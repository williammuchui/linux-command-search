/*implement basic commands
 * default lists all the commands available forf linux in the default file
 * add more features later
 * implement a help page,, man commands
 */

/*import*/
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*define variables */
#define DEFAULT_LINUX_PATH "linux"

/*declarations*/
void handle_default_case(void);
void read_from_linux_file(void);
void handle_commands(int argc, char** argv);

/*main function
 * entry point to the program
 * command options -l
 */
int main(int argc, char** argv) {
  switch (argc) {
    case 1:
      handle_default_case();
      break;
    case 2:
      handle_commands(argc, argv);
      break;
    default:
      perror("Too Many Arguments\n");
      return 1;
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

void handle_commands(int argc, char** argv) {
  if (argc > 2) {
    perror("Too many Arguments\n");
    return;
  }

  if (!strcmp(argv[1], "-l")) {
    handle_default_case();
    return;
  } else {
    perror("Wrong Argument\n");
    return;
  }
}
