import collections
import matplotlib.pyplot as plt
import argparse

def count_8bit_occurrences(file_path):
    # Open the file in binary mode
    with open(file_path, 'rb') as file:
        # Read the entire file as binary data
        file_content = file.read()
    
    # Count occurrences of each 8-bit combination (each byte)
    byte_counter = collections.Counter(file_content)
    
    # Convert the byte counter to a list of (byte, count) tuples
    byte_counts = sorted(byte_counter.items())
    
    # Extract byte values and their counts
    byte_values = [byte for byte, count in byte_counts]
    counts = [count for byte, count in byte_counts]
    
    # Display the results as a bar chart
    plt.figure(figsize=(10, 6))
    plt.bar(byte_values, counts, color='blue')
    plt.title(f'Occurrences of 8-bit Combinations in {file_path}')
    plt.xlabel('Byte Value (0-255)')
    plt.ylabel('Count')
    plt.xticks(range(0, 256, 16))  # Show x-ticks every 16 values
    plt.show()

def main():
    # Set up argument parsing
    parser = argparse.ArgumentParser(description="Count occurrences of 8-bit combinations in a file")
    parser.add_argument('file_path', type=str, help='Path to the input file')
    
    # Parse the arguments
    args = parser.parse_args()
    
    # Call the function with the provided file path
    count_8bit_occurrences(args.file_path)

if __name__ == "__main__":
    main()
