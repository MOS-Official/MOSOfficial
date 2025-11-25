// wrapper added (no comments touched)
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::cell::Cell; // this lets us change `next` safely
use core::sync::atomic::{AtomicUsize, Ordering}; //bullshit am keeping cell just bc i know this wont work
use spin::Mutex;

pub struct LockedAllocator(Mutex<BumpAllocator>);

unsafe impl GlobalAlloc for LockedAllocator {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    self.0.lock().alloc(layout)
  }
  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    self.0.lock().dealloc(ptr, layout)
  }
}


// original code unchanged below
pub struct BumpAllocator{ //the struct that gave me mental issues
  heap_start: usize,
  heap_end: usize,
  next: AtomicUsize, //Most annoying shit ever
}

impl BumpAllocator { //makes the allocator actually work i cant really explain it i barely understand structs myself
  pub unsafe fn set_heap(&mut self, start: usize, size: usize){
    self.heap_start = start;
    self.heap_end = start + size;
    self.next.store(start, Ordering::Relaxed); //dont ask i dont know either
    return; //it returns wow shocker
  }
}

unsafe impl GlobalAlloc for BumpAllocator { //adding properties to the struct
  unsafe fn alloc(&self, layout: Layout) -> *mut u8{
    let current = self.next.load(Ordering::Relaxed); 
    let alloc_start = align_up(current, layout.align()); //tidies up the memory
    let alloc_end = alloc_start + layout.size();

    if alloc_end > self.heap_end {
      return null_mut(); //no more memory and does nothing so like have fun i guess (returns null which is well null)
    }

    //if there is space then we update next so we dont push twice in the same place
    self.next.store(alloc_end, Ordering::Relaxed);

    //now we return the address of the memory it gives
    return alloc_start as *mut u8; //yes it has to be as POINTER mutable u8 
  }

  unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
    //Now i could make the dealocator
    //but my brother 
    //yeah am tired and not in the mood i can do it tomorrow
    //or you can do me a favor and do it <3
    return;
  }
}

fn align_up(address: usize, alignment: usize) -> usize {
//too much math man am tired
//anyways this just tidies our shit up
let aligned = (address + alignment - 1) & !(alignment - 1);

//GIVE IT BACK lol
return aligned;
}

//make shit global i wanna_cry pun intended
#[global_allocator]
pub static ALLOCATOR: LockedAllocator = LockedAllocator(Mutex::new(BumpAllocator {
//dont ask me what a static is i know everything breaks without it so i guess just keep it 
//but i know its job is to and i quote myself "make shit global"
  heap_start: 0,
  heap_end: 0,
  next: AtomicUsize::new(0),
}));

impl LockedAllocator { //dont tell me to organize please i barely know what am doing nvm i dont know at all
  pub unsafe fn set_heap(&self, start: usize, size: usize){
    self.0.lock().set_heap(start,size); //i dont know anything i am a failure of a programmer
  }
}




















