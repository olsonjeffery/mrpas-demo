use std::io::println;

pub struct Map {
    size: uint,
    tiles: ~[Tile]
}
impl Map {
    pub fn new(size: uint) -> Map {
        let length = size * size;
        let mut ctr = 0;
        let mut tiles = ~[];
        while ctr < length {
            tiles.push(Tile::new());
            ctr += 1;
        }
        Map { size: size, tiles: tiles }
    }
    pub fn example() -> Map {
        let map_str =
            "##############################################"+
            "#######################      #################"+
            "#####################    #     ###############"+
            "######################  ###        ###########"+
            "##################      #####             ####"+
            "################       ########    ###### ####"+
            "###############      #################### ####"+
            "################    ######                  ##"+
            "########   #######  ######   #     #     #  ##"+
            "########   ######      ###                  ##"+
            "########                                    ##"+
            "####       ######      ###   #     #     #  ##"+
            "#### ###   ########## ####                  ##"+
            "#### ###   ##########   ######################"+
            "#### ##################   #####          #####"+
            "#### ###             #### #####          #####"+
            "####           #     ####                #####"+
            "########       #     #### #####          #####"+
            "########       #####      ####################"+
            "##############################################";
        let mut map = Map::new(46);
        let mut ctr = 0;
        for c in map_str.chars() {
            match c {
                ' ' => map.tiles[ctr].allow_los = true,
                _ => {}
            }
            ctr += 1;
        }
        map
    }
    pub fn reset(&mut self) {
        for t in self.tiles.mut_iter() {
            t.visible = false;
        }
    }
    pub fn draw_to_stdout(&self, limit: (uint, uint), focus: (uint, uint)) {
        let mut out_strs = ~[];
        let mut y = 0;
        let (lx, ly) = limit;
        let (fx, fy) = focus;
        while y < self.size && y <= ly {
            let mut x = 0;
            let mut line = ~"";
            let yc = y * self.size;
            while x < self.size && x <= lx {
                let c = yc + x;
                let t = &self.tiles[c];
                if x == fx && y == fy {
                    line = line + "@";
                } else {
                    if !t.visible { line = line + "."; }
                    else if !t.allow_los { line = line + "#"; }
                    else { line = line+" "; }
                }
                x += 1;
            }
            out_strs.push(line);
            y += 1;
        }
        for s in out_strs.iter() {
            println(*s);
        }
    }
}

pub struct Tile {
    allow_los: bool,
    visible: bool
}

impl Tile {
    pub fn new() -> Tile {
        Tile { allow_los: false, visible: false }
    }
}

