import { instantiate, StyleUnit } from "./lib/taffy_layout.generated.js"; 

const {Node, TaffyTree} = await instantiate();

const tree = new TaffyTree();

const node = new Node(tree);

node.setWidth(10, StyleUnit.Px)

console.log(node.computeLayout(0).width);
