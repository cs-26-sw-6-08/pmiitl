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
The monitor will start automatically after restoring the backup. Open the frontend at:
```sh
http://localhost:3000
```
Violations of the properties (and errors) are shown in the output box, and are also written to the console

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
1. In the frontend at `http://localhost:3000`, edit the properties in the top box
2. Press **Push properties & restart monitor** (or Ctrl+Enter)
3. If the properties compile, the monitor restarts with them and they are saved to `program.txt`.
   If not, the error is shown in the output box and the previous monitor keeps running
4. Press **Stop monitor** to stop monitoring; pushing properties starts it again
