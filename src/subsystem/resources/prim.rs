use std::cell::{RefCell, RefMut};


pub const INVAILD_PRIM_HANDLE: i16 = -1;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum PrimType {
    #[default]
    PrimTypeNone = 0,
    PrimTypeGroup = 1,
    PrimTypeTile = 2,
    PrimTypeSprt = 4,
    PrimTypeText = 5,
    PrimTypeSnow = 7,
}


#[derive(Debug, Clone, Default)]
pub struct Prim {
    typ: PrimType,
    draw_flag: bool,
    alpha: i8,
    blend: bool,
	parent: i16,
    sprt: i16,
    grand_parent: i16,
    grand_son: i16,
    z: i16,
    x: i16,
    y: i16,
    w: i16,
    h: i16,
    u: i16,
    v: i16,
    opx: i16,
    opy: i16,
    rotation: i16,
    factor_x: i16,
    factor_y: i16,
    child: i16,
    group_args2: i16,
    mode: i16,
    tile: i16,
    text_index: i16,
    attr: u32,
}

impl Prim {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_type(&mut self, typ: PrimType) {
        self.typ = typ;
    }

    pub fn set_draw_flag(&mut self, draw_flag: bool) {
        self.draw_flag = draw_flag;
    }

    pub fn set_alpha(&mut self, alpha: i8) {
        self.alpha = alpha;
    }

    pub fn set_blend(&mut self, blend: bool) {
        self.blend = blend;
    }

    pub fn set_parent(&mut self, parent: i16) {
        self.parent = parent;
    }

    pub fn set_sprt(&mut self, sprt: i16) {
        self.sprt = sprt;
    }

    pub fn set_grand_parent(&mut self, grand_parent: i16) {
        self.grand_parent = grand_parent;
    }

    pub fn set_grand_son(&mut self, grand_son: i16) {
        self.grand_son = grand_son;
    }

    pub fn set_z(&mut self, z: i16) {
        self.z = z;
    }

    pub fn set_x(&mut self, x: i16) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i16) {
        self.y = y;
    }

    pub fn set_w(&mut self, w: i16) {
        self.w = w;
    }

    pub fn set_h(&mut self, h: i16) {
        self.h = h;
    }

    pub fn set_u(&mut self, u: i16) {
        self.u = u;
    }

    pub fn set_v(&mut self, v: i16) {
        self.v = v;
    }

    pub fn set_opx(&mut self, opx: i16) {
        self.opx = opx;
    }

    pub fn set_opy(&mut self, opy: i16) {
        self.opy = opy;
    }

    pub fn set_rotation(&mut self, rotation: i16) {
        self.rotation = rotation;
    }

    pub fn set_factor_x(&mut self, factor_x: i16) {
        self.factor_x = factor_x;
    }

    pub fn set_factor_y(&mut self, factor_y: i16) {
        self.factor_y = factor_y;
    }

    pub fn set_child(&mut self, child: i16) {
        self.child = child;
    }

    pub fn set_group_args2(&mut self, group_args2: i16) {
        self.group_args2 = group_args2;
    }

    pub fn set_mode(&mut self, mode: i16) {
        self.mode = mode;
    }

    pub fn set_tile(&mut self, tile: i16) {
        self.tile = tile;
    }

    pub fn set_text_index(&mut self, text_index: i16) {
        self.text_index = text_index;
    }

    pub fn apply_attr(&mut self, attr: u32) {
        self.attr |= attr;
    }

    pub fn set_attr(&mut self, attr: u32) {
        self.attr = attr;
    }

    pub fn get_type(&self) -> PrimType {
        self.typ
    }

    pub fn get_draw_flag(&self) -> bool {
        self.draw_flag
    }

    pub fn get_alpha(&self) -> i8 {
        self.alpha
    }

    pub fn get_blend(&self) -> bool {
        self.blend
    }

    pub fn get_parent(&self) -> i16 {
        self.parent
    }

    pub fn get_sprt(&self) -> i16 {
        self.sprt
    }

    pub fn get_grand_parent(&self) -> i16 {
        self.grand_parent
    }

    pub fn get_grand_son(&self) -> i16 {
        self.grand_son
    }

    pub fn get_z(&self) -> i16 {
        self.z
    }

    pub fn get_x(&self) -> i16 {
        self.x
    }

    pub fn get_y(&self) -> i16 {
        self.y
    }

    pub fn get_w(&self) -> i16 {
        self.w
    }

    pub fn get_h(&self) -> i16 {
        self.h
    }

    pub fn get_u(&self) -> i16 {
        self.u
    }

    pub fn get_v(&self) -> i16 {
        self.v
    }

    pub fn get_opx(&self) -> i16 {
        self.opx
    }

    pub fn get_opy(&self) -> i16 {
        self.opy
    }

    pub fn get_rotation(&self) -> i16 {
        self.rotation
    }

    pub fn get_factor_x(&self) -> i16 {
        self.factor_x
    }

    pub fn get_factor_y(&self) -> i16 {
        self.factor_y
    }

    pub fn get_child(&self) -> i16 {
        self.child
    }

    pub fn get_group_args2(&self) -> i16 {
        self.group_args2
    }

    pub fn get_mode(&self) -> i16 {
        self.mode
    }

    pub fn get_tile(&self) -> i16 {
        self.tile
    }

    pub fn get_text_index(&self) -> i16 {
        self.text_index
    }

    pub fn get_attr(&self) -> u32 {
        self.attr
    }
}


#[derive(Debug, Clone, Default)]
pub struct PrimManager {
    prims: Vec<RefCell<Prim>>,
}

impl PrimManager {
    pub fn new() -> Self {
        Self {
            // allocate 4096 prims
            prims: vec![RefCell::new(Prim::new()); 4096],
        }
    }

    pub fn get_prim(&self, id: i16) -> RefMut<'_, Prim> {
        self.prims[id as usize].borrow_mut()
    }

    pub fn prim_init_with_type(&mut self, id: i16, typ: PrimType) {
        let mut prim = self.get_prim(id);
        if prim.get_type() != typ {
            if prim.get_type() == PrimType::PrimTypeGroup {
                let mut child = prim.get_child();
                while child != INVAILD_PRIM_HANDLE {
                    self.unlink_prim(child);
                    child = self.get_prim(child).get_grand_son();
                }
            }

            prim.set_type(typ);
            prim.set_draw_flag(true);
            if typ == PrimType::PrimTypeGroup {
                prim.set_child(INVAILD_PRIM_HANDLE);
                prim.set_group_args2(INVAILD_PRIM_HANDLE);
                prim.set_x(0);
                prim.set_y(0);
            }
        }

        // prim.m_Attribute |= 0x40;
        prim.set_sprt(-1);
    }

    pub fn unlink_prim(&self, id: i16) {
        let parent = self.get_prim(id).get_parent();
        if parent != INVAILD_PRIM_HANDLE {
            // unlink previous parent and child
            let grand_parent = self.get_prim(id).get_grand_parent();
            if grand_parent == INVAILD_PRIM_HANDLE {
                let next_id = self.get_prim(id).get_grand_son();
                self.get_prim(parent).set_child(next_id);
            } else {
                let next_id = self.get_prim(id).get_grand_son();
                self.get_prim(grand_parent).set_grand_son(next_id);
            }
            
            let grand_son = self.get_prim(id).get_grand_son();
            let grand_parent = self.get_prim(id).get_grand_parent();
            if grand_son == INVAILD_PRIM_HANDLE {
                let parent = self.get_prim(id).get_parent();
                self.get_prim(parent).set_group_args2(grand_parent);
            } else {
                self.get_prim(grand_parent).set_grand_parent(grand_parent);
            }
            
            // self.prim_slots[idx].m_Attribute |= 0x40;
            self.get_prim(id).set_parent(INVAILD_PRIM_HANDLE);
        }
    }

    pub fn prim_move(&mut self, new_root: i32, id: i32) {
        self.unlink_prim(id as i16);
        let parent_id = self.get_prim(new_root as i16).get_parent();
        
        if parent_id != INVAILD_PRIM_HANDLE {
            let mut prim = self.get_prim(id as i16);
            prim.set_parent(parent_id);
            prim.set_grand_parent(new_root as i16);
            let mut root_prim = self.get_prim(new_root as i16);
            if root_prim.get_grand_son() == INVAILD_PRIM_HANDLE {
                prim.set_grand_son(INVAILD_PRIM_HANDLE);
                root_prim.set_grand_son(id as i16);
                let parent_id = root_prim.get_parent();
                let mut prim2 = self.get_prim(parent_id);
                prim2.set_group_args2(id as i16);
            } else {
                let grand_son = root_prim.get_grand_son();
                prim.set_grand_son(grand_son);
                root_prim.set_grand_son(id as i16);
            }
            // self.prim_slots[idx].m_Attribute |= 0x40;
        }
    }

    pub fn set_prim_group_in(&mut self, new_root: i32, id: i32) {
        self.prim_init_with_type(new_root as i16, PrimType::PrimTypeGroup);
        self.unlink_prim(id as i16);

        let mut prim = self.get_prim(id as i16);
        prim.set_parent(new_root as i16);
        prim.set_grand_son(INVAILD_PRIM_HANDLE);

        let mut root_prim = self.get_prim(new_root as i16);
        if root_prim.get_child() == INVAILD_PRIM_HANDLE {
            prim.set_grand_parent(INVAILD_PRIM_HANDLE);
            root_prim.set_child(id as i16);
        } else {
            let arg2 = root_prim.get_group_args2();
            prim.set_grand_parent(arg2);

            let arg2 = self.get_prim(arg2).get_group_args2();
            let mut prim2 = self.get_prim(arg2);
            prim2.set_grand_son(id as i16);
        }
        root_prim.set_group_args2(id as i16);
        // self.prim_slots[idx].m_Attribute |= 0x40;
    }

    pub fn prim_set_op(&mut self, id: i32, opx: i32, opy: i32) {
        let mut prim = self.get_prim(id as i16);
        prim.set_opx(opx as i16);
        prim.set_opy(opy as i16);
    }

    pub fn prim_set_alpha(&mut self, id: i32, alpha: i32) {
        let mut prim = self.get_prim(id as i16);
        prim.set_alpha(alpha as i8);
    }

    pub fn prim_set_blend(&mut self, id: i32, blend: i32) {
        let mut prim = self.get_prim(id as i16);
        prim.set_blend(blend != 0);
    }

    pub fn prim_set_draw(&mut self, id: i32, draw: i32) {
        let mut prim = self.get_prim(id as i16);
        prim.set_draw_flag(draw != 0);
    }

    pub fn prim_set_rotation(&mut self, id: i32, rotation: i32) {
        let mut prim = self.get_prim(id as i16);
        prim.set_rotation(rotation as i16);
    }

    pub fn prim_set_scale(&mut self, id: i32, factor_x: i32, factor_y: i32) {
        let mut prim = self.get_prim(id as i16);
        prim.set_factor_x(factor_x as i16);
        prim.set_factor_y(factor_y as i16);
    }

    pub fn prim_set_uv(&mut self, id: i32, u: i32, v: i32) {
        let mut prim = self.get_prim(id as i16);
        prim.set_u(u as i16);
        prim.set_v(v as i16);
    }

    pub fn prim_set_size(&mut self, id: i32, w: i32, h: i32) {
        let mut prim = self.get_prim(id as i16);
        prim.set_w(w as i16);
        prim.set_h(h as i16);
    }

    pub fn prim_set_pos(&mut self, id: i32, x: i32, y: i32) {
        let mut prim = self.get_prim(id as i16);
        prim.set_x(x as i16);
        prim.set_y(y as i16);
    }

    pub fn prim_set_sprt(&mut self, id: i32, sprt: i32) {
        let mut prim = self.get_prim(id as i16);
        prim.set_sprt(sprt as i16);
    }

    pub fn prim_set_z(&mut self, id: i32, z: i32) {
        let mut prim = self.get_prim(id as i16);
        prim.set_z(z as i16);
    }

    pub fn prim_set_mode(&mut self, id: i32, mode: i32) {
        let mut prim = self.get_prim(id as i16);
        prim.set_mode(mode as i16);
    }

    pub fn prim_set_text(&mut self, id: i32, text_index: i32) {
        let mut prim = self.get_prim(id as i16);
        prim.set_text_index(text_index as i16);
    }

    pub fn prim_set_tile(&mut self, id: i32, tile: i32) {
        let mut prim = self.get_prim(id as i16);
        prim.set_tile(tile as i16);
    }

    pub fn prim_add_attr(&mut self, id: i32, mask: u32) {
        let mut prim = self.get_prim(id as i16);
        let attr = prim.get_attr();
        prim.set_attr(attr | mask);
    }

    pub fn prim_remove_attr(&mut self, id: i32, mask: u32) {
        let mut prim = self.get_prim(id as i16);
        let attr = prim.get_attr();
        prim.set_attr(attr & mask);
    }

    pub fn prim_set_attr(&mut self, id: i32, attr: i32) {
        let mut prim = self.get_prim(id as i16);
        prim.set_attr(attr as u32);
    }

    pub fn prim_get_type(&self, id: i32) -> PrimType {
        self.get_prim(id as i16).get_type()
    }

}