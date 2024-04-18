

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include <glib.h>

int main(void){
  
  // Declare and Initialize some variables
  GHashTable * wordcounts = g_hash_table_new_full(g_str_hash, g_str_equal,g_free,g_free);
  char buf[256] = {0};
  
  // Read words and insert into hash table
  while (scanf("%255s", buf) != EOF)
    g_hash_table_insert(wordcounts, g_strdup(buf), NULL);
  
  //print the result
  printf("unique words: %d\n", g_hash_table_size(wordcounts));
  
  // cleanup
  g_hash_table_destroy(wordcounts); 
  return 0;
}
