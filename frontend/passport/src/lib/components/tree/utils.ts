interface NodeValue {
  h: string
}

interface Node extends NodeValue {
  children: Node[]
}

const flattenNodes = (nodes: Node[]): string[] => nodes.flatMap(node => [node.h, ...flattenNodes(node.children)])

export { type NodeValue, type Node, flattenNodes }
