#!/usr/bin/env python3

# Import the subprocess module
import subprocess
import difflib

# Run the rust project using cargo
rust_result = subprocess.run(["cargo", "run"], cwd="regex_rust", stdout=subprocess.PIPE)
# Check the return code and output
if rust_result.returncode == 0:
    rust_output = rust_result.stdout.decode().splitlines()
else:
    print("Rust project failed")
    print(rust_result.stderr)

# Run the .NET project using dotnet cli
dotnet_result = subprocess.run(
    ["dotnet", "run"], cwd="regex_dotnet", stdout=subprocess.PIPE
)
# Check the return code and output
if dotnet_result.returncode == 0:
    dotnet_output = dotnet_result.stdout.decode().splitlines()
else:
    print(".NET project failed")
    print(dotnet_result.stderr)

if None not in (rust_output, dotnet_output):
    # Compare the outputs using difflib
    diff = difflib.ndiff(rust_output, dotnet_output)

    # Print the comparison results
    print("Comparison results:")
    for line in diff:
        print(line)

else:
    print("Couldn't parse application output")
