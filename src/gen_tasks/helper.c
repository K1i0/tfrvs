#include <stdio.h>
#include <stdlib.h>
#include <math.h>
//в аргументе указать r = 1024 или 4096
int main( int argc, char *argv[ ] ){
    int r_max, m;
    int t_max = 100;
    r_max = atoi(argv[1]);
    //m = atoi(argv[2]);
    m = 500;
    printf("r_max = %d\n",r_max);
    FILE* ptr;
    for(int j = m; j <= 5000; j = j + 500){
        if(j == 500) {
            ptr = fopen("m_500_task.txt", "w");
            for(int i = 0; i < j; i++){
                fprintf(ptr, "%d;%d;%d\n", i+1, rand() % (r_max + 1 - 0) + 0, rand() % (t_max + 1 - 0) + 0);
            }
        } else if (j == 1000) {
            ptr = fopen("m_1000_task.txt", "w");
            for(int i = 0; i < j; i++){
                fprintf(ptr, "%d;%d;%d\n", i+1, rand() % (r_max + 1 - 0) + 0, rand() % (t_max + 1 - 0) + 0);
            }
        } else if (j == 1500) {
            ptr = fopen("m_1500_task.txt", "w");
            for(int i = 0; i < j; i++){
                fprintf(ptr, "%d;%d;%d\n", i+1, rand() % (r_max + 1 - 0) + 0, rand() % (t_max + 1 - 0) + 0);
            }
        } else if (j == 2000) {
            ptr = fopen("m_2000_task.txt", "w");
            for(int i = 0; i < j; i++){
                fprintf(ptr, "%d;%d;%d\n", i+1, rand() % (r_max + 1 - 0) + 0, rand() % (t_max + 1 - 0) + 0);
            }
        } else if (j == 2500) {
            ptr = fopen("m_2500_task.txt", "w");
            for(int i = 0; i < j; i++){
                fprintf(ptr, "%d;%d;%d\n", i+1, rand() % (r_max + 1 - 0) + 0, rand() % (t_max + 1 - 0) + 0);
            }
        } else if (j == 3000) {
            ptr = fopen("m_3000_task.txt", "w");
            for(int i = 0; i < j; i++){
                fprintf(ptr, "%d;%d;%d\n", i+1, rand() % (r_max + 1 - 0) + 0, rand() % (t_max + 1 - 0) + 0);
            }
        } else if (j == 3500) {
            ptr = fopen("m_3500_task.txt", "w");
            for(int i = 0; i < j; i++){
                fprintf(ptr, "%d;%d;%d\n", i+1, rand() % (r_max + 1 - 0) + 0, rand() % (t_max + 1 - 0) + 0);
            }
        } else if (j == 4000) {
            ptr = fopen("m_4000_task.txt", "w");
            for(int i = 0; i < j; i++){
                fprintf(ptr, "%d;%d;%d\n", i+1, rand() % (r_max + 1 - 0) + 0, rand() % (t_max + 1 - 0) + 0);
            }
        } else if (j == 4500) {
            ptr = fopen("m_4500_task.txt", "w");
            for(int i = 0; i < j; i++){
                fprintf(ptr, "%d;%d;%d\n", i+1, rand() % (r_max + 1 - 0) + 0, rand() % (t_max + 1 - 0) + 0);
            }
        } else if (j == 5000) {
            ptr = fopen("m_5000_task.txt", "w");
            for(int i = 0; i < j; i++){
                fprintf(ptr, "%d;%d;%d\n", i+1, rand() % (r_max + 1 - 0) + 0, rand() % (t_max + 1 - 0) + 0);
            }
        } else {
            ptr = fopen("m_TEST_task.txt", "w");
            for(int i = 0; i < j; i++){
                fprintf(ptr, "%d;%d;%d\n", i+1, rand() % (r_max + 1 - 0) + 0, rand() % (t_max + 1 - 0) + 0);
            }
        }
        
        if (ptr == NULL) { 
            printf("Error Occurred While creating a "
                "file !"); 
            exit(1); 
        }
    }
    
    // for(int i = 0; i < m; i++){
    //     fprintf(ptr, "%d;%d;%d\n", i+1, rand() % (r_max + 1 - 0) + 0, rand() % (t_max + 1 - 0) + 0);
    // }

    //(rand() % (upper - lower + 1)) + lower; 

 /*
 вывод должен иметь вид: 
 
 id;r;t

rj ∈{1, 2, …, n}, 
tj ∈{1, 2, …, 100}.
 */   
fclose(ptr);
return 0;
}