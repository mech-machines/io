extern crate crossbeam_channel;
use mech_core::{hash_string, TableIndex, Table, Value, ValueType, ValueMethods, Transaction, Change, TableId, Register};
use mech_utilities::{Machine, MachineRegistrar, RunLoopMessage};
//use std::sync::mpsc::{self, Sender};
use std::thread::{self};
use crossbeam_channel::Sender;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

lazy_static! {
  static ref IO__STREAMS_OUT: u64 = hash_string("io-streams/out");
}

export_machine!(io__streams_out, io__streams_out_reg);

extern "C" fn io__streams_out_reg(registrar: &mut dyn MachineRegistrar, outgoing: Sender<RunLoopMessage>) -> Vec<Change> {
  registrar.register_machine(Box::new(Out{outgoing}));
  vec![
    Change::NewTable{table_id: *IO__STREAMS_OUT, rows: 0, columns: 0},
  ]
}

#[derive(Debug)]
pub struct Out {
  outgoing: Sender<RunLoopMessage>,
}

impl Machine for Out {

  fn name(&self) -> String {
    "io-streams/out".to_string()
  }

  fn id(&self) -> u64 {
    Register{table_id: TableId::Global(*IO__STREAMS_OUT), row: TableIndex::All, column: TableIndex::All}.hash()
  }

  fn on_change(&mut self, table: &Table) -> Result<(), String> {
    for i in 1..=table.rows {
      for j in 1..=table.columns {
        let (value, _) = table.get_unchecked(i,j);
        match value.value_type() {
          ValueType::String => {
            print!("{}",table.get_string_from_hash(value).unwrap());
          }
          ValueType::Quantity => {
            print!("{}",value.as_f64().unwrap());
          }
          ValueType::Boolean => {
            print!("{}",value.as_bool().unwrap());
          }
          ValueType::NumberLiteral => {
            // TODO print number literals
          }
          _ => (), // No output representation for other value types
        }
      }
      io::stdout().flush().unwrap();
    }
    Ok(())
  }
}