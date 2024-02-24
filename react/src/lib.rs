use std::{
    collections::{HashMap, HashSet},
    sync::atomic::{AtomicU64, Ordering},
};

fn input_id() -> InputCellId {
    static ID: AtomicU64 = AtomicU64::new(0);
    InputCellId(ID.fetch_add(1, Ordering::Relaxed))
}

fn compute_id() -> ComputeCellId {
    static ID: AtomicU64 = AtomicU64::new(0);
    ComputeCellId(ID.fetch_add(1, Ordering::Relaxed))
}

fn callback_id() -> CallbackId {
    static ID: AtomicU64 = AtomicU64::new(0);
    CallbackId(ID.fetch_add(1, Ordering::Relaxed))
}

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(u64);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(u64);

struct InputCell<T> {
    value: T,
    listeners: HashSet<ComputeCellId>,
}

struct ComputeCell<'a, T> {
    value: T,
    deps: Vec<CellId>,
    f: Box<dyn Fn(&[T]) -> T + 'a>,
    cbs: HashSet<CallbackId>,
}

impl<'a, T: Copy + PartialEq> ComputeCell<'a, T> {

    fn compute(&self, args: &[T]) -> T {
        (self.f)(args)
    }


}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub enum ComputeError {
    InvalidDependency(CellId),
    NonexistentComputeId(ComputeCellId),
}

pub struct Reactor<'a, T> {
    // Just so that the compiler doesn't complain about an unused type parameter.
    // You probably want to delete this field.
    inputs: HashMap<InputCellId, InputCell<T>>,
    computes: HashMap<ComputeCellId, ComputeCell<'a, T>>,
    listeners: HashMap<CellId, HashSet<ComputeCellId>>,
    callbacks: HashMap<CallbackId, Box<dyn FnMut(T) + 'a>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            inputs: HashMap::new(),
            computes: HashMap::new(),
            listeners: HashMap::new(),
            callbacks: HashMap::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let input_id = input_id();
        self.inputs.insert(
            input_id,
            InputCell {
                value: initial,
                listeners: HashSet::new(),
            },
        );
        input_id
    }

    fn check(&self, deps: &[CellId]) -> Result<(), CellId> {
        for id in deps {
            match id {
                r @ CellId::Input(id) => { if !self.inputs.contains_key(id) {return Err(*r)}},
                r @ CellId::Compute(id) => { if !self.computes.contains_key(id) {return Err(*r)}},
            }
        }
        Ok(())
    }

    fn args(&self, deps: &[CellId]) -> Vec<T> {
        deps.iter()
            .map(|id| match id {
                CellId::Input(id) => self.inputs[id].value,
                CellId::Compute(id) => self.computes[id].value,
            })
            .collect()
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        // check dependencies
        self.check(dependencies)?;

        // create compute cell
        let compute_id = compute_id();
        let compute_cell = ComputeCell {
            value: compute_func(&self.args(dependencies)),
            f: Box::new(compute_func),
            deps: dependencies.to_owned(),
            cbs: HashSet::new(),
        };
        self.computes.insert(compute_id, compute_cell);

        // listeners
        for id in dependencies {
            let e = self.listeners.entry(*id).or_insert_with(HashSet::new);
            e.insert(compute_id);
            // self.listeners.insert(*id, HashSet::from([compute_id]));
        }
        Ok(compute_id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(id) => self.inputs.get(&id).map(|i| i.value),
            CellId::Compute(id) => self.computes.get(&id).map(|c| c.value),
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        if let None = self.inputs.get(&id) {
            return false;
        }
        let cell = self.inputs.get_mut(&id).unwrap();
        if cell.value == new_value {
            return true;
        }
        cell.value = new_value;
        // propagate update
        if let None = self.listeners.get(&CellId::Input(id)) {
            return true;
        }
        let listeners = self.listeners.get(&CellId::Input(id)).unwrap();
        for id in listeners.clone() {
            let c: &ComputeCell<_> = self.computes.get(&id).unwrap();
            let cbs = c.cbs.clone();
            let origin = c.value;
            let v = self.compute(id);
            if Some(origin) != v {
                cbs.iter().for_each(|cb| self.callbacks.get_mut(&cb).unwrap()(v.unwrap()));
            }
            
        }
        // if let Some(sets) = self.listeners.get(&CellId::Input(id)) {
        //     for id in sets {
        //         let c: &ComputeCell<_> = self.computes.get(id).unwrap();
        //         let args = self.dep_value(*id);
        //         let v = c.compute(&args);
        //         self.compute(*id);

        //         // run callbacks
        //         if v != c.value {
        //             c.cbs.iter().for_each(|cb| self.callbacks.get_mut(&cb).unwrap()(v));
        //         }
        //     }
        // }
        true
    }

    fn dep_value(&self, id: ComputeCellId) -> Vec<T> {
        let cell = self.computes.get(&id).unwrap();
        cell.deps
            .iter()
            .map(|id| self.value(*id).unwrap())
            .collect::<Vec<_>>()
    }

    pub fn compute(&mut self, id: ComputeCellId) -> Option<T> {
        if let None = self.computes.get(&id) {
            return None;
        }
        let args = self.dep_value(id);
        let c: &mut ComputeCell<T> = self.computes.get_mut(&id).unwrap();
        let origin = c.value;
        let value = c.compute(&args);
        c.value = value;
        if value != origin {
            let listeners = self.listeners.get(&CellId::Compute(id));
            if let Some(sets) = listeners {
                sets.clone().iter().for_each(|id| {self.compute(*id);});
            }
        }
        Some(value)
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        let cell = self.computes.get_mut(&id)?;
        let cid = callback_id();
        cell.cbs.insert(cid);
        self.callbacks.insert(cid, Box::new(callback));
        Some(cid)
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        if let Some(c) = self.computes.get_mut(&cell) {
            if c.cbs.remove(&callback) {
                return Ok(());
            }
            return Err(RemoveCallbackError::NonexistentCallback);
        }
        Err(RemoveCallbackError::NonexistentCell)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id() {
        let cid = callback_id();
        assert_eq!(cid.0, 0);
        let cid = callback_id();
        assert_eq!(cid.0, 1);
    }
}
