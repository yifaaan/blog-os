use x86_64::{
    registers::control::Cr3,
    structures::paging::{page_table::FrameError, FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};


/// Returns a mutable reference to the active level 4 page table
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    // read the current value of the CR3 register
    // which contains the physical address of the level 4 page table
    let (level_4_table_frame, _) = Cr3::read();
    let phys = level_4_table_frame.start_address();
    // get the virtual address of the level 4 page table
    let virt = physical_memory_offset + phys.as_u64();

    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

// pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
//     translate_addr_inner(addr, physical_memory_offset)
// }

/// Translate a virtual address to a physical address by walking the page table chain
// fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
//     // get the level 4 page table frame from the CR3 register
//     let (level_4_table_frame, _) = Cr3::read();

//     let table_indexes = [
//         addr.p4_index(),
//         addr.p3_index(),
//         addr.p2_index(),
//         addr.p1_index(),
//     ];

//     let mut frame = level_4_table_frame;

//     for &index in table_indexes.iter() {
//         // get the page table start virtual address
//         let virt = physical_memory_offset + frame.start_address().as_u64();
//         let table_ptr: *const PageTable = virt.as_ptr();
//         let table = unsafe { &*table_ptr };

//         let entry = &table[index];
//         // update the table frame to the next level
//         frame = match entry.frame() {
//             Ok(f) => f,
//             Err(FrameError::FrameNotPresent) => return None,
//             Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
//         }
//     }
//     Some(frame.start_address() + addr.page_offset().into())
// }

/// Initialize a new OffsetPageTable.
///
/// # Safety
///
/// This function is unsafe because it may cause a page fault.
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}


pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;
    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe {
        // FIXME: only for testing
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    // flush TLB
    map_to_result.expect("map_to failed").flush();
}


pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        None
    }
}
