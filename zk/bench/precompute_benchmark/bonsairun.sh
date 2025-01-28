#!/bin/bash

# TODO: change these global variables
# export BONSAI_API_KEY=
# export BONSAI_API_URL=

# should use precompute in this benchmark run or not
use_precompute="true"
# number of iterations to get an average over
iteration_count=10

# Function to calculate the average of an array and return the average value
calculate_average() {
  local arr=("$@")   # Get all arguments as an array
  local sum=0
  local count=${#arr[@]}  # Number of elements in the array

  # Check if the array is empty
  if [ $count -eq 0 ]; then
    echo "Array is empty. Cannot calculate average."
    return 1
  fi

  # Iterate through the array to calculate the sum
  for num in "${arr[@]}"; do
    sum=$((sum + num))
  done

  # Calculate the average (integer division)
  local average=$((sum / count))

  # Return the average
  echo "$average"
}

main() {
    # store the time each iteration took to run
    runtimes=()

    for iteration_count in $(seq 1 $iteration_count); do
        if [ "$use_precompute" = true ]; then
            program_output=$(cargo run precompute)
        else
            program_output=$(cargo run)
        fi
        iteration_runtime=$(echo "$program_output" | grep -oP '\d+(?=\sseconds)')
        echo iteration_runtime
        runtimes+=("$iteration_runtime")
    done

    # Print the captured output
    average_value=$(calculate_average "${runtimes[@]}")
    echo "The average time taken to run is $average_value" seconds
}

# call the main function
main
# call the main function