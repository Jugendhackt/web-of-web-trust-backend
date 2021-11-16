# Total-Graph

Let set of all domains be $D_{t}$ (Total domains). We define a set of root
domains $D_{r}$ with $D_{r} \subseteq D_{t}$.

Certain processes (`scrapers`) will create a relation over all domains and
create a new relation $E_{t} = D_{t} \times D_{t}$. This relation may have
mutliple, redundant links from a domain to another domain. We will assign $d_l$
as the count of relations from a domain to all other domains. We will now reduce
all redundant edges to a final edge for each link in $E_{t}$ and weigh the link
by the count of edges from a domain to another domain proportional to $d_l$. The
resulting weight of an edge may be referred to as $e_l$.

At this point all edges that are part of a circle reference need to be removed
to achieve:
$\exists! e_1, e_2, e_3 … e_n \in E_t \,.\, (e_1, e_2) \land (e_2, e_3) \land … (e_n, e_a)$.

Let $G_t$ (Total Graph) be a directed, weighted graph of all domains with
$D_{t}$ (Total domains) over $E_{t}$.

Let $n \in \N$ be the root value of trust. We will assign each node in $D_r$ a
weight of $n$. All other domains will have the base value of $0.0$.

To evaluate the scores all links from the domains in $D_r$ will be visited and
the linked domain will have the linking domain's value times the weight of the
edge ($n * e_l\,$) to the domain added to it's value.

After this first iteration all domains in $D_r$ will be marked.

Now execute the following steps until all domains on the graph are marked:

Select all unmarked domains $D_u$ in $D_t$, where all domains that link to them
are marked.

To evaluate the scores all links from the domains in $D_u$ will be visited and
the linked domain will have the linking domain's value times the weight of the
edge ($n * e_l\,$) to the domain added to it's value. All domains in $D_u$ will
be marked.

This may also be expressed with a tri-color algorithm were a marked node is
referred to as gray and a node were all in- and out linking edges are to marked
nodes as well as the node itself as black node.

Once all domains are marked, we recieve the weighted, directed acyclic graph
$G_h$ with the vertices $D_t$ and the edges $E_t$. This problem is most likely
NP-hard since you need to remove circular references.

## Accelerating

1. Shortcut paths: wikipedia -> john's blog -[only link]> sinclair's blog -[only
   link]> grace's blog as shortcut terminating end
2. Don't go from root but instead from target -> referencing domains -> ... ->
   root
3. A node that has man out-referencing links (with a high depth for links) or is
   in root set may be cached in the hot-graph
4. Direct cache for requests with redis
5. Cache common hotspot paths and duplicates (a -> d -> c; b -> c => b -> and
   discard path (not value) or d->c)

## Cycle removal algorithm

Given set of root nodes $D_r$ and set of all nodes as $D_t$ with
$D_r \subset D_t$.

Get all links from $D_r$ as $L_{u}$ and mark nodes.

Iteraitvely weight all links in $L_u$ that reference nodes in $D_u$ and weight +
mark them.

Iteratively weight all remaining unmarked linked nodes and mark all remaining
nodes in $D_u$. Build new link set for $L_u$ from linked, unmarked nodes of
nodes in $D_u$

```
dr = [...]

fn decycle(domain, path):
   for links of domain:
      if links.target in path:
         delete links
      else:
         decycle(links.target, path + [domain])


for domain in dr:
   decycle(domain)
```

Create a path -> advance the path by going steps and split up on each bend ->
When link that points to node in path is met => remove link
