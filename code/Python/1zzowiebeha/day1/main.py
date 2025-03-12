import os

# todo: tests,
#       data safety,
#       use generators,
#       test file-type bytes to validate data file

############
# Settings #
# Place the data file in the same directory as the program file #

DATA_FILENAME = 'data.txt'
BASE_FILE_PATH = os.path.join(os.path.dirname(__file__))
DATA_FILE_PATH = os.path.join(BASE_FILE_PATH, DATA_FILENAME)

############


def parse_data() -> tuple[list[int], list[int]]:
    """Parse a space-deliminated 2-column file of integers into two lists."""
    list1 = []
    list2 = []
    try:
        with open(DATA_FILE_PATH, 'r') as file_object:
            for line in file_object.readlines():
                data = line.split(' ')

                # todo: handle missing column
                
                try:
                    line_column1 = int(data[0])
                    line_column2 = int(data[-1].strip())
                except ValueError:
                    print("\n[!] Error: non-integer data found. Offending line:")
                    print(f"'{line}'")
                    print("Skipping to next available line...")
                finally:
                    list1.append(line_column1)
                    list2.append(line_column2)
                
                
    except FileNotFoundError:
        print(f"\n[!] Error: File '{DATA_FILENAME}' could not be found under the following path: \n{BASE_FILE_PATH}")

    return (list1, list2)


def caclulate_differences(l1: list[int], list2: list[int]) -> int:
    """For each smallest value of each list,
    pop, and find the difference between the two.
    Return the sum of all differences."""
    sum = 0

    for iteration in range(len(list1)):
        # possible todo?: turn into generator
        list1_nextmin = list1.pop( list1.index(min(list1)) )
        list2_nextmin = list2.pop( list2.index(min(list2)) )
            
        pair = ( list1_nextmin, list2_nextmin )
            
        difference = max(pair) - min(pair)
            
        sum += difference
        
    return sum
    
    
if __name__ == "__main__":
    list1, list2 = parse_data()
    
    if list1 and list2:
        sum = caclulate_differences(list1, list2)
        
        print(f"\nSum of all differences: {sum}")