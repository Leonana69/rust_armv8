// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
// pub struct Frame {
//     number: usize,
// }

// #[derive(Debug)]
// pub struct memoryArea {
// 	base_addr: 	u64,
// 	length:		u64,
// }

// pub const PAGE_SIZE: usize = 4096;

// impl Frame {
//     fn containing_address(address: usize) -> Frame {
//         Frame{ number: address / PAGE_SIZE }
//     }
// }

// pub trait FrameAllocator {
//     fn allocate_frame(&mut self) -> Option<Frame>;
//     fn deallocate_frame(&mut self, frame: Frame);
// }

// pub struct AreaFrameAllocator {
//     next_free_frame: Frame,
//     current_area: memoryArea,
//     // areas: MemoryAreaIter,
//     kernel_start: Frame,
//     kernel_end: Frame,
//     multiboot_start: Frame,
//     multiboot_end: Frame,
// }

// impl FrameAllocator for AreaFrameAllocator {
//     fn allocate_frame(&mut self) -> Option<Frame> {
//         if let Some(area) = self.current_area {
//             // "Clone" the frame to return it if it's free. Frame doesn't
//             // implement Clone, but we can construct an identical frame.
//             let frame = Frame{ number: self.next_free_frame.number };

//             // the last frame of the current area
//             let current_area_last_frame = {
//                 let address = area.base_addr + area.length - 1;
//                 Frame::containing_address(address as usize)
//             };

//             if frame > current_area_last_frame {
//                 // all frames of current area are used, switch to next area
//                 self.choose_next_area();
//             } else if frame >= self.kernel_start && frame <= self.kernel_end {
//                 // `frame` is used by the kernel
//                 self.next_free_frame = Frame {
//                     number: self.kernel_end.number + 1
//                 };
//             } else if frame >= self.multiboot_start && frame <= self.multiboot_end {
//                 // `frame` is used by the multiboot information structure
//                 self.next_free_frame = Frame {
//                     number: self.multiboot_end.number + 1
//                 };
//             } else {
//                 // frame is unused, increment `next_free_frame` and return it
//                 self.next_free_frame.number += 1;
//                 return Some(frame);
//             }
//             // `frame` was not valid, try it again with the updated `next_free_frame`
//             self.allocate_frame()
//         } else {
//             None // no free frames left
//         }
//     }

 //     fn deallocate_frame(&mut self, frame: Frame) {
//         // TODO (see below)
//     }
// }

extern crate core;
macro_rules! BITFIELD {
    ($base:ident $field:ident : $fieldtype:ty [
        $($thing:ident $set_thing:ident[$r:expr],)+]) => {
        impl $base {$(
            #[inline]
            pub fn $thing(&self) -> $fieldtype {
                let size = $crate::core::mem::size_of::<$fieldtype>() * 8;
                self.$field << (size - $r.end) >> (size - $r.end + $r.start)
            }
            #[inline]
            pub fn $set_thing(&mut self, val: $fieldtype) {
                let mask = ((1 << ($r.end - $r.start)) - 1) << $r.start;
                self.$field &= !mask;
                self.$field |= (val << $r.start) & mask;
            }
        )+}
    }
}

struct mmu_level2_table_descriptor {
    BitFields: u64
}

BITFIELD!(mmu_level2_table_descriptor BitFields: u64 [
    value1 set_value1[0..2],
    ignored1 set_ignored1[2..16],
    tableAddress set_tableAddress[16..48],
    reserved0 set_reserved0[48..52],
    ignored2 set_ignored2[52..59],
    PXNTable set_PXNTable[59..60],
    UXNTable set_UXNTable[60..61],
    APTable set_APTable[61..63],
    NSTable set_NSTable[63..64],
]);