// Minimal mock for script_macros_common.hpp
#define QUOTE(var1) #var1
#define ARR_1(ARG1) {ARG1}
#define ARR_2(ARG1,ARG2) {ARG1,ARG2}
#define ARR_3(ARG1,ARG2,ARG3) {ARG1,ARG2,ARG3}
// Add other macros if your specific tests depend on them
#define DEBUG_SYNCHRONOUS 0
