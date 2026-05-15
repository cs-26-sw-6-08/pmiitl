# PMIITL
## Setup using docker compose:

#### Step 1 - Define properties
open `program.txt` and define using the specification language the properties you wish to monitor

#### Step 2 - Build & start
```sh
docker compose up
```
#### Step 3 - Restore backup
1. navigate to:
```sh
http://localhost:8123
```
2. Click to upload/restore the backup
3. Select the included `restore_backup.tar`

#### Step 4 - Monitoring
The monitor will start automatically in the console after restoring the backup

Violations of the properties will be written to the console

#### Step 5 - Log in to Home Assistant
username: `demo`

password: `demo`

When signed in, all demo devices' states can be modified in the dashboard

Furthermore automations simulating the demo homes device states can be found/changed in:
```sh
http://localhost:8123/config/automation/dashboard
```
or by navigating to Settings -> Automations & scenes

#### Step 6 - Changing the monitored properties
1. open `program.txt` and define using the specification language the properties you wish to monitor
2. using a second terminal enter:
```sh
docker compose restart monitor
```
3. the monitor should now output violations occurring
