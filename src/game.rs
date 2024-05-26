use crate::{
    input::InputHandler,
    render::{self, RenderGroup, WindowState},
};

const TILE_WIDTH: f32 = 0.5;
const TILE_HEIGHT: f32 = 0.5;
const X_OFFSET: f32 = 0.25;
const Y_OFFSET: f32 = 0.25;
const X_SPRITE_INDEX: usize = 1;
const O_SPRITE_INDEX: usize = 2;

// also represents a player, where Empty means neither
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TileState {
    X,
    O,
    Empty,
}

impl TileState {
    fn get_sprite(&self) -> usize {
        match self {
            Self::X => X_SPRITE_INDEX,
            Self::O => O_SPRITE_INDEX,
            Self::Empty => panic!("Attempted reading sprite index of Empty Tile"),
        }
    }
}

#[derive(Debug)]
struct Board {
    tiles: [[TileState; 3]; 3],
}

impl Board {
    fn check_column_win(&self, column: usize) -> bool {
        let r1 = self.tiles[0][column];
        let r2 = self.tiles[1][column];
        let r3 = self.tiles[2][column];
        r1 != TileState::Empty && r1 == r2 && r2 == r3
    }

    fn check_row_win(&self, row: usize) -> bool {
        let c1 = self.tiles[row][0];
        let c2 = self.tiles[row][1];
        let c3 = self.tiles[row][2];
        c1 != TileState::Empty && c1 == c2 && c2 == c3
    }

    fn check_left_diagonal(&self) -> bool {
        let t1 = self.tiles[0][0];
        let t2 = self.tiles[1][1];
        let t3 = self.tiles[2][2];
        t1 != TileState::Empty && t1 == t2 && t2 == t3
    }

    fn check_right_diagonal(&self) -> bool {
        let t1 = self.tiles[0][2];
        let t2 = self.tiles[1][1];
        let t3 = self.tiles[2][0];
        t1 != TileState::Empty && t1 == t2 && t2 == t3
    }

    fn update_tile(&mut self, tile: [usize; 2], state: TileState) {
        self.tiles[tile[1]][tile[0]] = state;
    }
}

struct Hitbox {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    sprite: usize,
}

pub struct GameState {
    sprites: Vec<wgpu::BindGroup>,
    ents: Vec<Hitbox>,
    board: Board,
    current_player: TileState,
    turns: u32,
}

impl GameState {
    pub fn new(sprites: Vec<wgpu::BindGroup>) -> Self {
        Self {
            sprites,
            ents: Vec::new(),
            board: Board {
                tiles: [[TileState::Empty; 3]; 3],
            },
            current_player: TileState::O,
            turns: 0,
        }
    }

    pub fn register_ent(&mut self, x: f32, y: f32, width: f32, height: f32, sprite: usize) {
        let hit = Hitbox {
            x,
            y,
            width,
            height,
            sprite,
        };
        self.ents.push(hit);
    }

    pub fn get_meshes<'a>(&self, ws: &WindowState) -> Vec<RenderGroup> {
        let mut v = Vec::with_capacity(self.ents.len());
        for e in &self.ents {
            v.push(render::make_quad(
                [e.x, e.y],
                e.width,
                e.height,
                self.sprites.get(e.sprite).unwrap(),
                &ws,
            ));
        }
        v
    }

    pub fn update(&mut self, inp: &InputHandler) {
        let mpos = inp.get_screen_mouse_position();
        if !inp.lmb
            || mpos[0].abs() as f32 > 1.0 - X_OFFSET
            || mpos[1].abs() as f32 > 1.0 - Y_OFFSET
        {
            return;
        }
        let x_position = ((mpos[0] as f32 - X_OFFSET + 1.0) / TILE_WIDTH) as usize;
        let y_position = ((mpos[1] as f32 - Y_OFFSET + 1.0) / TILE_HEIGHT) as usize;

        if self.board.tiles[y_position][x_position] != TileState::Empty {
            return;
        }
        self.register_ent(
            x_position as f32 * TILE_WIDTH + X_OFFSET - 1.0,
            y_position as f32 * TILE_HEIGHT + Y_OFFSET - 0.5,
            TILE_WIDTH,
            TILE_HEIGHT,
            self.current_player.get_sprite(),
        );
        self.board
            .update_tile([x_position, y_position], self.current_player);

        if self.board.check_row_win(y_position)
            || self.board.check_column_win(x_position)
            || self.board.check_left_diagonal()
            || self.board.check_right_diagonal()
        {
            panic!("Game won")
        }

        self.current_player = if self.current_player == TileState::X {
            TileState::O
        } else {
            TileState::X
        };
        self.turns += 1;
    }
}
