#include "array.h"
#include <stdio.h>
#include <stdlib.h>

Linked_List *new_linked_list(void) {
  Linked_List *linked_list = malloc(sizeof(Linked_List));

  linked_list->first = NULL;
  linked_list->size = 0;

  return linked_list;
}

Linked_List *linked_list_push(Linked_List *linked_list, void *item) {
  Linked_List_Node *new_last_node = malloc(sizeof(Linked_List_Node));
  new_last_node->value = item;
  new_last_node->next = NULL;
  if (linked_list->first == NULL) {
    linked_list->first = new_last_node;
  } else {
    Linked_List_Node *last_node =
        linked_list_get_node(linked_list, linked_list->size - 1);
    last_node->next = new_last_node;
  }
  linked_list->size++;
  return linked_list;
}

Linked_List *linked_list_pop(Linked_List *linked_list) {
  Linked_List_Node *second_to_last_node =
      linked_list_get_node(linked_list, linked_list->size - 2);
  free(second_to_last_node->next);
  second_to_last_node->next = NULL;
  return linked_list;
}

void *linked_list_get(Linked_List *linked_list, int index) {
  return linked_list_get_node(linked_list, index)->value;
}

Linked_List_Node *linked_list_get_node(Linked_List *linked_list, int index) {
  if (linked_list->first == NULL) {
    fprintf(stderr, "Linked List is empty\n");
    exit(1);
  }
  int i = 0;
  Linked_List_Node *current_node = linked_list->first;
  while (current_node->next != NULL && i < index) {
    current_node = current_node->next;
    i++;
  }

  return current_node;
}
