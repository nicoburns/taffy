import { instantiate } from "./lib/taffy_layout.generated.js";

const { Node, TaffyTree } = await instantiate();

const tree = new TaffyTree();

const node = new Node(tree, { width: 10, height: 10 });
const node2 = new Node(tree, { width: 5, height: 5, marginTop: 5 });
node.addChild(node2);
console.log(node.computeLayout(100).width);
console.log(node.computeLayout(100).height);
console.log(node.computeLayout(100).x);
console.log(node.computeLayout(100).y);
console.log(node.computeLayout(100).child(0).width);
console.log(node.computeLayout(100).child(0).height);
console.log(node.computeLayout(100).child(0).x);
console.log(node.computeLayout(100).child(0).y);
