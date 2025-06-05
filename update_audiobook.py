import re
import sys

def update_audiobook_initializations(file_path):
    with open(file_path, 'r') as file:
        content = file.read()
    
    # Pattern to match Audiobook initializations
    pattern = r'Ok\(Audiobook\s*\{[^}]*?\}\)'
    
    def replace_audiobook(match):
        # Check if 'selected' is already in the initialization
        if 'selected:' in match.group(0):
            return match.group(0)
        
        # Add the selected field before the closing brace
        return match.group(0).replace('})', ',\n                        selected: false,\n                    })')
    
    # Replace all occurrences
    updated_content = re.sub(pattern, replace_audiobook, content)
    
    with open(file_path, 'w') as file:
        file.write(updated_content)
    
    print(f"Updated {file_path}")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        update_audiobook_initializations(sys.argv[1])
    else:
        print("Please provide a file path")