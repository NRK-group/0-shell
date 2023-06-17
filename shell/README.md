# Command

## Todo

### Command required to be implemented

- [x] echo
- [x] cd
- [x] ls
- [x] pwd
- [x] cat
- [x] cp
- [x] rm
- [x] mv
- [x] mkdir
- [x] exit
- [x] Ctrl + D

### Improve

    - Test
        most of the test only test the arguments. Need to test the excute function as well.
    - Error handling
        most of the error handling is handle by the OS. Need to customise it or improve it.
    - Auto complete 
        need to implement auto complete for the command. When user press tab, it will auto complete the command if it is in the path.
    - History 
        need to implement history for the command. When user press up arrow, it will show the previous command. This is currently implement using rustyline crate But i want to implement it myself in the future.
