#ifndef ARRAY_H
#define ARRAY_H

typedef struct {
  void *value;
  void *next;
} Linked_List_Node;

typedef struct {
  int size;
  Linked_List_Node *first;
} Linked_List;

Linked_List *new_linked_list(void);
Linked_List *linked_list_push(Linked_List *linked_list, void *item);
Linked_List *linked_list_pop(Linked_List *linked_list);
void *linked_list_get(Linked_List *linked_list, int index);
Linked_List_Node *linked_list_get_node(Linked_List *linked_list, int index);

#endif // !ARRAY_H
