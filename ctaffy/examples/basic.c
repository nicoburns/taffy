#include <stdlib.h>
#include <stdint.h>
#include "taffy.h"

int main() {
    // Create tree
    TaffyTree *tree = TaffyTree_New();

    // Create child (+set styles)
    TaffyNodeId child = TaffyTree_NewNode(tree).value;
    TaffyStyle *child_style = TaffyTree_GetStyleMutRef(tree, child).value;
    TaffyStyle_SetWidth(child_style, 0.5, STYLE_VALUE_UNIT_PERCENT);
    TaffyStyle_SetHeight(child_style, 0, STYLE_VALUE_UNIT_AUTO);

    // Create parent (+set styles)
    TaffyNodeId parent = TaffyTree_NewNode(tree).value;
    TaffyStyle *parent_style = TaffyTree_GetStyleMutRef(tree, parent).value;
    TaffyStyle_SetWidth(parent_style, 100, STYLE_VALUE_UNIT_LENGTH);
    TaffyStyle_SetHeight(parent_style, 100, STYLE_VALUE_UNIT_LENGTH);
    TaffyStyle_SetJustifyContent(parent_style, TAFFY_ALIGN_CONTENT_CENTER);

    // Setup parent-child relationship
    TaffyTree_AppendChild(tree, parent, child);

    // Compute layout + print result
    TaffyTree_ComputeLayout(tree, parent);
    TaffyTree_PrintTree(tree, parent);

    // Free tree
    TaffyTree_Free(tree);
    return 0;
}
