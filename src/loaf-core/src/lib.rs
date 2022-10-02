use std::rc::Rc;

use loaf_span::Span;
use term::*;
use types::Level;
use value::{Closure, Env, Stuck, Value};

pub mod term;
pub mod types;
pub mod value;

impl Closure {
    pub fn apply(&self, name: Option<String>, arg: Rc<Value>) -> Rc<Value> {
        self.term.eval(&self.env.add(name, arg))
    }
}

impl Stuck {
    pub fn quote(&self, base: Level) -> Rc<Term> {
        match self {
            Stuck::Rigid(cur_depth) => Rc::new(Term::Var(Var {
                range: Span::Generated,
                index: Level::to_index(base, cur_depth.clone()),
            })),
        }
    }
}

impl Value {
    pub fn quote(&self, depth: Level) -> Rc<Term> {
        match self {
            Value::Neutral(stuck, spine) => Rc::new(Term::App(App {
                range: Span::Generated,
                head: stuck.quote(depth),
                spine: spine.iter().map(|x| x.quote(depth)).collect(),
            })),
            Value::Lam(binder, body) => Rc::new(Term::Lambda(Lambda {
                range: Span::Generated,
                binder: binder.clone(),
                body: body.apply(Some(binder.clone()), Rc::new(Value::var(depth))).quote(depth.inc()),
            })),
            Value::Pi(binder, ty, body) => Rc::new(Term::Pi(Pi {
                range: Span::Generated,
                binder: binder.clone(),
                typ: ty.quote(depth),
                body: body.apply(binder.clone(), Rc::new(Value::var(depth))).quote(depth.inc()),
            })),
            Value::Sigma(binder, ty, body) => Rc::new(Term::Sigma(Sigma {
                range: Span::Generated,
                binder: binder.clone(),
                typ: ty.quote(depth),
                body: body.apply(binder.clone(), Rc::new(Value::var(depth))).quote(depth.inc()),
            })),
            Value::Pair(fst, snd) => Rc::new(Term::Pair(Pair {
                range: Span::Generated,
                fst: fst.quote(depth),
                snd: snd.quote(depth),
            })),
            Value::Left(expr) => Rc::new(Term::Left(Left {
                range: Span::Generated,
                term: expr.quote(depth),
            })),
            Value::Right(expr) => Rc::new(Term::Right(Right {
                range: Span::Generated,
                term: expr.quote(depth),
            })),
            Value::Universe => todo!(),
        }
    }
}

impl App {
    pub fn eval(&self, env: &Env) -> Rc<Value> {
        let mut head = self.head.eval(env);
        for arg in &self.spine {
            let arg = arg.eval(env);
            head = match &*head {
                Value::Neutral(stuck, sp) => Rc::new(Value::Neutral(
                    stuck.clone(),
                    [sp.as_slice(), &self.spine.iter().map(|x| x.eval(env)).collect::<Vec<Rc<Value>>>()].concat(),
                )),
                Value::Lam(name, closure) => closure.apply(Some(name.clone()), arg),
                _ => unreachable!("Internal Error: Malformed application"),
            }
        }
        head
    }
}

impl Term {
    pub fn eval(&self, env: &Env) -> Rc<Value> {
        match self {
            Term::Let(term) => {
                let val = term.val.eval(env);
                term.then.eval(&env.add(Some(term.binder.clone()), val))
            },
            Term::Universe(_) => Rc::new(Value::Universe),
            Term::Var(term) => env.vars.get(term.index.0).expect("Internal Error: Malformed Term").clone(),
            Term::Pi(term) => Rc::new(Value::Pi(term.binder.clone(), term.typ.eval(env), Closure::new(env, &term.body))),
            Term::Lambda(term) => Rc::new(Value::Lam(term.binder.clone(), Closure::new(env, &term.body))),
            Term::App(term) => term.eval(env),
            Term::Pair(term) => Rc::new(Value::Pair(term.fst.eval(env), term.snd.eval(env))),
            Term::Sigma(term) => Rc::new(Value::Sigma(term.binder.clone(), term.typ.eval(env), Closure::new(env, &term.body))),
            Term::Left(term) => Rc::new(Value::Left(term.term.eval(env))),
            Term::Right(term) => Rc::new(Value::Right(term.term.eval(env))),
        }
    }
}
