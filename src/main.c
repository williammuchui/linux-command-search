/*implement basic commands
 * default lists all the commands available forf linux in the defaul file
 * add more features later
 */

/*import*/
#include <stdio.h>
#include <stdlib.h>

/*define variables */
#define DEFAULT_LINUX_PATH "linux"

/*structs*/
struct Data {
  char command[20];
  char description[100];
};

/*declarations*/
void handle_default_case(void);
void read_from_linux_file(void);

/*main function
 * entry point to the program
 */
int main(int argc, char** argv) {
  switch (argc) {
    case 1:
      handle_default_case();
      break;
    case 2:
      printf("Yet to be implemented;\n");
      break;
    default:
      printf("Yet to be implemented;\n");
      break;
  }
  return 0;
}

void handle_default_case() {
  /*read from default linux file*/
  read_from_linux_file();
}

void read_from_linux_file(void) {
  FILE* linux_file = fopen(DEFAULT_LINUX_PATH, "r");
  if (!linux_file) {
    perror("Error opening linux file!\n");
    return;
  }

  fseek(linux_file, 0, SEEK_END);
  long linux_file_size = ftell(linux_file);
  fseek(linux_file, 0, SEEK_SET);

  /*allocate memory*/
  char* linux_file_contents = (char*)malloc(linux_file_size + 1);
  if (!linux_file_contents) {
    perror("Memory allocation for linux file contents failed;\n");
    fclose(linux_file);
    return;
  }

  size_t bytes_read_from_linux_file =
      fread(linux_file_contents, 1, linux_file_size, linux_file);
  if (ferror(linux_file)) {
    perror("Error reading linux file contents;\n");
    free(linux_file_contents);
    fclose(linux_file);
    return;
  }

  linux_file_contents[linux_file_size] = '\0';
  printf("%s", linux_file_contents);
  free(linux_file_contents);
  fclose(linux_file);
  return;
}
