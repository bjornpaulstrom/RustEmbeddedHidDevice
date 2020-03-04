//! TRust libribray
//! Per Lindgren, per.lindgren@ltu.se
//! macros.rs
//! trust macro api
// convert logic priority 1..16 to hardware priority for the m3/m4,
// assuming 4 bits prio field (16 prioirity levels)
// the idle process runs at logic priority 0 (conceptually a task never running to completion)
// in the idle process you may imlpment a threading concurrency model

#[macro_export]
// should perhaps be a function
macro_rules! hw_prio {
  ($logic_prio: expr) => ( interpolate_idents! {
//    (0x100 - ($logic_prio << 4))
	(((1 << 4) - $logic_prio) << 4) 
  } )
}
// convert hardware priority to logic piority 1..15
// not finished
// #[macro_export]
// macro_rules! sw_prio {
//   ($logic_prio: expr) => ( interpolate_idents! {
//     (0x100 - ($logic_prio << 4))
//   } )
// }

use core::intrinsics::atomic_singlethreadfence;

// #[inline(always)]
// pub fn compiler_barrier() {
//     unsafe { asm!( "" :::: "volatile"); }
// }

#[inline(always)]
pub fn compiler_barrier() {
    unsafe {
        atomic_singlethreadfence();
    };
}

#[macro_export]
#[cfg(feature = "klee")]
macro_rules! resource {
    ($x: ident, $y: ident, $z: expr) => ( interpolate_idents! {
        #[inline(never)] #[allow(non_snake_case)] fn [claim_ $x _dummy]() { compiler_barrier() }
        #[inline(never)] #[allow(non_snake_case)] fn [unclaim_ $x _dummy]() { compiler_barrier() }
        #[allow(non_camel_case_types)] struct [Phantom $x] {}
        static $x: Res< $y, [Phantom $x] > = Res::new($z, [ $x _CEILING]);
    } )
}

// claim_dummy / unclaim_dummy will be opted away under cargo build --release
#[macro_export]
#[cfg(not(feature = "klee"))]
macro_rules! resource {
    ($x: ident, $y: ty, $z: expr) => ( interpolate_idents! {
        #[inline(always)] #[allow(non_snake_case)] fn [claim_ $x _dummy]() { compiler_barrier() }
        #[inline(always)] #[allow(non_snake_case)] fn [unclaim_ $x _dummy]() { compiler_barrier() }
        #[allow(non_camel_case_types)] struct [Phantom $x] {}
        static $x: Res< $y, [Phantom $x] > = Res::new($z, [ $x _CEILING]);
    } )
}

#[macro_export]
macro_rules! claim {
    ($x: ident, $y: tt, $z: stmt) => ( interpolate_idents! {
        {
            [claim_ $x _dummy]();
            let $y = $x.claim();
            $z
            drop($y);
            [unclaim_ $x _dummy]();
        } // here critical section ends
    } )
}

#[macro_export]
macro_rules! claim_mut {
    ($x:ident, $y:ident, $z:stmt) => ( interpolate_idents! {
        {
            [claim_ $x _dummy]();
            let mut $y = $x.claim();
            $z
            drop($y);
            [unclaim_ $x _dummy]();
        } // here critical section ends
    } )
}

// opt_claim will not affect basepri
#[macro_export]
macro_rules! opt_claim {
    ($x: ident, $y: tt, $z: stmt) => ( interpolate_idents! {
        {
            [claim_ $x _dummy]();
            let $y = $x.opt_claim();
            $z
            drop($y);
            [unclaim_ $x _dummy]();
        } // here critical section ends
    } )
}

#[macro_export]
macro_rules! opt_claim_mut {
    ($x:ident, $y:ident, $z:stmt) => ( interpolate_idents! {
        {
            [claim_ $x _dummy]();
            let mut $y = $x.opt_claim();
            $z
            drop($y);
            [unclaim_ $x _dummy]();
        } // here critical section ends
    } )
}

#[macro_export]
macro_rules! critical {
    ($x:ident, $z:stmt) => ( interpolate_idents! {
        {
            [claim_ $x _dummy]();
            let _dummy = $x.claim(); // to pevent premature Drop, to _* prevent warning
            $z
            drop(_dummy);
            [unclaim_ $x _dummy]();
        } // here critical section ends
    } )
}

// Experimental version for optimized accesss
// macro_rules! trans_claim_mut {
//     ($x:ident, $y:ident, $z:stmt) => ( interpolate_idents! {
//         {
//             unsafe {
//             let mut $y =
//             (core::mem::transmute::<Res<Vector,PhantomA>, OptRes<Vector,PhantomA>> (A)).claim();
//             $z
//             drop($y);
//         };
//         } // here critical section ends
//     } )
// }


#[macro_export]
macro_rules! task {
  ($task_id: ident, $task_irq: expr, $task_prio: expr, $task_body: stmt) => ( interpolate_idents! {
      #[allow(non_upper_case_globals)] pub const [TASK_PRIO_ $task_id] : u32 = $task_prio;
      #[allow(non_upper_case_globals)] pub const [TASK_IRQ_ $task_id] : usize = $task_irq;
      pub extern "C" fn $task_id () { $task_body }
  } )
}
