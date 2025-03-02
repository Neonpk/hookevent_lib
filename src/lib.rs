/*
	Author: Neonpk (Chichard)
    Links: 
        Github: https://github.com/neonpk
        Steam:  https://steamcommunity.com/id/neonpk
    Event system from GLua on Rust
*/

use once_cell::sync::Lazy;

use std::{collections::HashMap, convert::TryInto, sync::Mutex};

type TCallback<T, TOut> = Box<dyn FnMut(&T) -> Option<TOut> + Send>;
type THandlers<T,TOut> = HashMap<String, Vec<TCallback<T,TOut>>>;

trait GenericStatic<T, TOut> {
    const INSTANCE: Lazy<Mutex<THandlers<T,TOut>>> = Lazy::new(|| {
        let mut _m: THandlers<T,TOut> = HashMap::new();
        Mutex::new(_m)
    });
}
impl<T, TOut, U: ?Sized> GenericStatic<T, TOut> for U {}

pub struct Hook<T, TOut>(Lazy<Mutex<THandlers<T,TOut>>>);

impl<T, TOut> Hook<T, TOut> {

    pub const fn init() -> Self {
       Self( <Self as GenericStatic<T, TOut>>::INSTANCE )
    }

    
    pub fn get_handlers(&'static self) -> &'static Lazy<Mutex<THandlers<T,TOut>>> {
        let hooks = &self.0;
        hooks
    }
    
    pub fn add(&'static self, name: String, callback: impl FnMut(&T) -> Option<TOut> + Send + 'static) -> usize {
        let handlers = self.get_handlers();

        match handlers.lock(){
            Ok(mut mp) => {

                if !mp.contains_key(&name){
                    #[allow(unused_mut)]
                    let mut new_vec: Vec<TCallback<T, TOut>> = Vec::new();
                    mp.insert(name.clone(), new_vec);
                }

                match mp.get_mut(&name){
                    Some(vec) => { 
                        vec.push(Box::new(callback));  
                        return <usize as TryInto<usize>>::try_into(vec.len()).unwrap() - 1;       
                    },
                    None => panic!("[HOOK Error]: Failed to get access to vector!")
                }

            }
            _ => panic!("[HOOK Error] Failed to get access to HashMap")
        }

    }

    pub fn call(&'static self, name: String, data: &T) -> Option<TOut> {
        #[allow(unused_mut)]
        let mut result: Option<TOut> = None;

        let handlers = self.get_handlers();

        match handlers.lock(){
            
            Ok(mut mp) => {
                
                if !mp.contains_key(&name) { return result; }
                
                match mp.get_mut(&name) {
                    Some(vec) => {
                        vec.into_iter().for_each(|f| {
                            result = f(data);
                        })
                    },
                    None => panic!("[HOOK Error]: Failed to get access to vector!")
                }

                return result;
            }
            _ => panic!("[HOOK Error]: Failed to get access HashMap!")
        }

    }

    pub fn remove(&'static self, name: String, id: usize) -> Result<(),String> {

        let handlers = self.get_handlers();

        match handlers.lock() {
            Ok(mut mp) => {

                let value = mp.get(&name).unwrap().get(id);

                if mp.contains_key(&name){
                    match value {
                        Some(_) => {
                            #[allow(unused_must_use)] {
                                mp.get_mut(&name).unwrap().remove(id.try_into().unwrap());
                            }
                            return Ok(());
                        },
                        None => { 
                            return Err(
                                format!(
                                "[HOOK Error]: Failed to remove hook<{}, {}>[{}][{}], bacause instance of event not found.", 
                                std::any::type_name::<T>(), std::any::type_name::<TOut>(), name, id 
                                ))
                        }
                    }
                }

                Err("[HOOK Error]: Failed to remove hook.".to_string())

            }
            _ => panic!("[HOOK Error]: Failed to get access HashMap")
        }

    }
}