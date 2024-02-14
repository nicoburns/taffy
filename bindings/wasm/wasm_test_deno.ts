import { instantiate, StyleUnit } from "jsr:@loading/taffy@0.1.1";

const { Node, TaffyTree } = await instantiate();

const tree = new TaffyTree();

const node = new Node(tree);

node.setWidth(10, StyleUnit.Px);
node.setHeight(10, StyleUnit.Px);

const node2 = new Node(tree);
node2.setWidth(5, StyleUnit.Px);
node2.setHeight(5, StyleUnit.Px);
node2.setMarginTop(5, StyleUnit.Px);
node.addChild(node2);
console.log(node.computeLayout(100).child(0).width);
console.log(node.computeLayout(100).child(0).height);
console.log(node.computeLayout(100).child(0).x);
console.log(node.computeLayout(100).child(0).y);
