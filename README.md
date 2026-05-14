# PMIITL
## Setup using docker compose:

#### Step 0 - Define properties
open `program.txt` and define using the specification language the properties you wish to monitor 

#### Step 1 - Build & start
```sh
docker compose up
```
#### Step 2 - Restore backup
1. navigate to:
```sh
https://localhost:8123
```
2. Click to restore the backup
3. Select the included `restore_backup.tar`

#### Step 3 - Monitoring
the monitor should now output violations occurring

#### Step 4 - Changing the monitored properties
1. open `program.txt` and define using the specification language the properties you wish to monitor
2. using a second terminal enter
```sh
docker compose restart monitor
```
3. the monitor should not output violations occurring
