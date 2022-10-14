import json
import os
import re
import signal
import subprocess
import sys
import time

from dataclasses import dataclass
from datetime import timedelta, datetime


def tail(filename, wait_duration=timedelta(milliseconds=100),
         line_timeout=timedelta(seconds=10)):
  """Tail a file and return its contents line by line.

  wait_duration specifies how long to wait between consecutive attempts at
  checking if anything new was written.
  line_timeout specifies how long overall to wait for each new line. If a line
  doesn't get writtn within this time, TimeoutError is raised.
  """
  with open(filename, 'r') as file:
    while True:
      line = ''
      started_line = datetime.now()
      while not line.endswith('\n'):
        if datetime.now() - started_line > line_timeout:
          raise TimeoutError(f'Waited for a line for longer than {line_timeout}')
        chunk = file.readline()
        if chunk is not None:
          line += chunk
        time.sleep(wait_duration.seconds)
      yield line


emulator_host_pattern = re.compile(
  r"│ (?P<name>\S*)\s*│ (?P<host>\S*)\s*│ (?P<emulator_ui>\S*)\s*│")
def parse_emulator_addresses(firebase_output_file, timeout=timedelta(minutes=1)):
  """Tails Firebase's output file to extract addresses of the emulators.

  Currently for auth and database emulators only.
  Returns a dict with them keyed by the name they appear in Firebase's output.
  Example:
    {
      'Authentication': 'localhost:9099',
      'Database': 'localhost:9000',
    }
  Raises TimeoutError if given lines are not found within the provided timeout.
  """
  required_emulators = ['Authentication', 'Database']
  start_time = datetime.now()
  result = {}
  try:
    for line in tail(firebase_output_file):
      match = re.match(emulator_host_pattern, line)
      if match is not None:
        result[match.group('name')] =  match.group('host')
      if all(key in result for key in required_emulators):
        return result
      if datetime.now() - start_time > timeout:
        raise TimeoutError(f'Did not find emulator addresses in {timeout}')
  except Exception as e:
    raise ValueError('Could not parse Firebase output for host addreses') from e


def create_config(emulator_hosts):
  """Creates and writes config of the app to .env file."""
  config = json.dumps({
    'auth_host': f"http://{emulator_hosts['Authentication']}/",
  })
  with open('.env', 'w') as env_file:
    env_file.write(config)


def clean_up(firebase_process):
  print('Cleaning up...')
  if firebase_process is not None:
    print('Killing Firebase emulators...')
    os.killpg(os.getpgid(firebase_process.pid), signal.SIGTERM)
    firebase_process.wait()
  print('Cleaned up')


def run_with_timeout(cmd, timeout=timedelta(minutes=1)):
  process = subprocess.Popen(cmd, preexec_fn=os.setsid)
  try:
    process.wait(timeout=timeout.seconds)
  except TimeoutExired as e:
    print(f'{cmd} did not finish in time: {timeout}. Killing...')
    os.killpg(os.getpgid(process.pid), signal.SIGKILL)
  return process.returncode


@dataclass
class Result:
  failed: bool
  message: str


def main():

  result = Result(failed=False, message='')
  try:
    print('Starting Firebase emulators...')

    firebase_process = None
    with open('firebase_output', 'w') as firebase_output, open('firebase_error', 'w') as firebase_error:
      firebase_process = subprocess.Popen(
        ['firebase', 'emulators:start', '--only', 'auth,database', '--project',
         'testing'],
        stdout=firebase_output, stderr=firebase_error, preexec_fn=os.setsid)

    emulator_hosts = parse_emulator_addresses(firebase_output.name)

    print(f'Firebase Emulators started, addresses: {emulator_hosts}')

    create_config(emulator_hosts)

    print(f'Config written to .env. Running cargo test...')

    cargo_test_return = run_with_timeout(['cargo', 'test'])
    if cargo_test_return != 0:
      result.failed = True
      result.message += f'cargo test failed with {cargo_test_return}; '

    firebase_status = firebase_process.poll()
    if firebase_status is not None:
      result.failed = True
      result.message += f'firebase emulators failed with {firebase_status}; '

  finally:
    clean_up(firebase_process)

  if result.failed:
    print(result.message)

  sys.exit(1 if result.failed else 0)

main()

