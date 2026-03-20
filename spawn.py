import os
import subprocess
import time
import json
from pathlib import Path

# THE MASTER ORCHESTRATOR / UNIVERSAL CONSTRUCTOR
def run_commune_ecosystem(root_dir, model_name):
    root = Path(root_dir).expanduser()
    log_file = root / "data" / "logs" / "commune_events.jsonl"

    # 1. THE GENESIS: Launch the first instance of the commune
    print(f"[GENESIS] Launching the Brave New Commune on {model_name}...")
    subprocess.Popen([
        "python3", "bravenewcommune.py",
        "--root", str(root),
        "--model", model_name,
        "--day", "1"
    ])

    print(f"[SYSTEM] Von Neumann Monitor Active on {log_file}")

    # 2. THE MITOSIS LOOP: Watch for the 'REPLICATE' signal
    while True:
        if log_file.exists():
            with open(log_file, "r") as f:
                lines = f.readlines()
                # Count current spawns to stay under the population cap
                population = len([l for l in lines if "SPAWN_EVENT" in l])

                # Check if the last log entry contains the 'REPLICATE' command
                if lines and "REPLICATE" in lines[-1] and population < 20:
                    new_agent_name = f"SubAgent_{population + 1}"
                    print(f"[MITOSIS] Trigger detected. Spawning {new_agent_name}...")

                    # Launch a fresh instance as a separate OS process
                    subprocess.Popen([
                        "python3", "bravenewcommune.py",
                        "--root", str(root),
                        "--model", model_name,
                        "--day", "1"
                    ])

                    # Record the birth in the events log
                    with open(log_file, "a") as event_log:
                        event_log.write(json.dumps({
                            "timestamp": time.time(),
                            "event": "SPAWN_EVENT",
                            "agent": new_agent_name
                        }) + "\n")

        time.sleep(10) # Metabolic rate: Check every 10 seconds

if __name__ == "__main__":
    # Ensure the logs directory exists so the monitor doesn't choke
    Path("~/Brave_New_Commune/data/logs").expanduser().mkdir(parents=True, exist_ok=True)
    run_commune_ecosystem("~/Brave_New_Commune", "gpt-oss:20b")
