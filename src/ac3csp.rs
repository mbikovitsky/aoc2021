use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use itertools::Itertools;

pub trait AC3CSP {
    type Variable: Eq + Hash + Copy;
    type Domain: Eq + Hash + Copy;

    fn domains(&self) -> &HashMap<Self::Variable, HashSet<Self::Domain>>;

    fn eval_unary_constraint(
        &self,
        variable: &Self::Variable,
        value: &Self::Domain,
    ) -> bool;

    fn eval_binary_constraint(
        &self,
        first: (&Self::Variable, &Self::Domain),
        second: (&Self::Variable, &Self::Domain),
    ) -> bool;

    fn all_binary_constraints(&self) -> &HashSet<(Self::Variable, Self::Variable)>;

    fn solve(&self) -> Option<HashMap<Self::Variable, HashSet<Self::Domain>>> {
        // https://en.wikipedia.org/w/index.php?title=AC-3_algorithm&oldid=961958002

        // Initial domains are made consistent with unary constraints.
        let mut domains = self.domains().clone();
        for (variable, domain) in domains.iter_mut() {
            domain.retain(|value| self.eval_unary_constraint(&variable, value));
        }

        // 'worklist' contains all arcs we wish to prove consistent or not.
        let mut worklist = self.all_binary_constraints()
            .iter()
            .map(|(x, y)| vec![(x, y), (y, x)])
            .flatten()
            .collect();

        while let Some((x, y)) = pop(&mut worklist) {
            if arc_reduce(self, &mut domains, *x, *y) {
                if (&self.domains()[&x]).is_empty() {
                    return None;
                }

                let new_arcs = self.all_binary_constraints()
                    .iter()
                    .filter(|(x, y)| x != y)
                    .map(|(x, y)| vec![(x, y), (y, x)])
                    .flatten()
                    .unique();
                worklist.extend(new_arcs);
            }
        }

        Some(domains)
    }
}

fn arc_reduce<C, V, D>(csp: &C, domains: &mut HashMap<V, HashSet<D>>, x: V, y: V) -> bool
    where
        V: Eq + Hash + Copy,
        D: Eq + Hash + Copy,
        C: AC3CSP<Variable=V, Domain=D> + ?Sized
{
    let constraint: Box<dyn Fn(&(D, D)) -> bool> =
        if csp.all_binary_constraints().contains(&(x, y)) {
            Box::new(|&(vx, vy)| csp.eval_binary_constraint((&x, &vx), (&y, &vy)))
        } else {
            Box::new(|&(vx, vy)| csp.eval_binary_constraint((&y, &vy), (&x, &vx)))
        };

    let to_remove: Vec<_> = (&domains[&x])
        .iter()
        .filter_map(|&vx| {
            let vy = (&domains[&y]).iter().find(|&&vy| constraint(&(vx, vy)));
            if let None = vy {
                Some(vx)
            } else {
                None
            }
        })
        .collect();
    let change = !to_remove.is_empty();

    for vx in to_remove {
        domains.get_mut(&x).unwrap().remove(&vx);
    }

    change
}

fn pop<T>(set: &mut HashSet<T>) -> Option<T>
    where
        T: Eq + Hash + Copy
{
    let value = *set.iter().next()?;
    set.remove(&value);
    Some(value)
}
