use bevy::ecs::query::QueryEntityError;
use bevy::prelude::Plugin as BevyPlugin;
use bevy::{prelude::*, utils::HashMap};
use bevy_ecs_tilemap::{MapQuery, Tile, TilePos};
use bevy_tileset_map::prelude::{TileId, TilePlacer, Tilesets};

pub struct Plugin;
impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(pathfinding).add_system(path_highlight);
    }
}

#[derive(Default, Debug, Clone, Component, PartialEq)]
pub struct TilePath(pub Vec<TilePos>);

#[derive(Default, Debug, Clone, Component, PartialEq)]
pub struct Destination {
    pub start: TilePos,
    pub goal: TilePos,
}

impl Destination {
    pub fn new(start: TilePos, goal: TilePos) -> Self {
        Destination { start, goal }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Node {
    pub pos: TilePos,
    pub score: u32,
    pub heuristic_score: u32,
    pub visited: bool,
    pub previous_pos: Option<TilePos>,
}

impl Node {
    pub fn new(pos: TilePos) -> Self {
        Node {
            pos,
            score: u32::MAX,
            heuristic_score: u32::MAX,
            visited: false,
            previous_pos: None,
        }
    }
}

pub fn path_highlight(
    tiles: Query<(&Tile, &TilePos)>,
    tile_paths: Query<&TilePath>,
    mut placer: TilePlacer,
    tilesets: Tilesets,
) {
    let mut tile_map = HashMap::<&TilePos, bool>::default();

    // Make a lookup for every TilePos that is in a TilePath
    for path in tile_paths.iter() {
        for tp in path.0.iter() {
            tile_map.insert(tp, true);
        }
    }

    if let Some(tileset) = tilesets.get_by_name("terrain") {
        let tileset_id = tileset.id().clone();

        if let (Some(highlight_id), Some(default_id)) = (
            tileset.get_tile_group_id("sand"),
            tileset.get_tile_group_id("dirt"),
        ) {
            for (_, tp) in tiles.iter() {
                let id = if tile_map.contains_key(tp) {
                    highlight_id
                } else {
                    default_id
                };
                placer
                    .replace(TileId::new(*id, tileset_id), *tp, 0, 0)
                    .err();
            }
        }
    }
}

pub fn pathfinding(
    query: Query<(Entity, &Destination)>,
    tile_query: Query<(Entity, &Tile, &TilePos)>,
    mut map_query: MapQuery,
    mut commands: Commands,
) {
    for (e, d) in query.iter() {
        if map_query.get_tile_entity(d.goal, 0, 0).is_err()
            || map_query.get_tile_entity(d.start, 0, 0).is_err()
        {
            info!("! 1");
            commands.entity(e).remove::<Destination>();
            continue;
        }
        let mut graph: HashMap<TilePos, Node> = HashMap::default();

        // Init Graph
        graph.insert(
            d.start,
            Node {
                pos: d.start,
                score: 0,
                heuristic_score: 0,
                ..Default::default()
            },
        );

        for (_, _, tp) in tile_query.iter() {
            if *tp != d.start && !graph.contains_key(tp) {
                let n = Node::new(*tp);
                graph.insert(*tp, n);
            }
        }

        loop {
            // Find first best
            let curr_node = graph
                .values()
                .into_iter()
                .fold(Node::new(TilePos(0, 0)), |acc, item| {
                    if !item.visited && item.heuristic_score < acc.heuristic_score {
                        *item
                    } else {
                        acc
                    }
                })
                .clone();

            if curr_node.pos == TilePos(0, 0) && curr_node.score == u32::MAX {
                // No path found, bail
                commands.entity(e).remove::<Destination>();
                continue;
            }

            // Set to visited
            graph
                .get_mut(&curr_node.pos)
                .expect("set curr_node visited")
                .visited = true;

            // Get neighbor tile entities
            for e in map_query
                .get_tile_neighbors(curr_node.pos, 0, 0)
                .iter()
                .filter_map(|n| n.ok())
            {
                // Check the node for that tile
                let tp = tile_query.get(e);
                if let Err(QueryEntityError::NoSuchEntity) = tp {
                    info!("No Such entity {:?}", curr_node);
                    continue;
                }
                let tp = tile_query.get(e).expect("neighbor node").2.clone();
                if tp == curr_node.pos {
                    continue;
                }

                if let Some(mut nn) = graph.get_mut(&tp) {
                    // Ignore tiles we have already checked
                    if nn.visited == false {
                        // Calc score based on current node
                        let new_score = calculate_score(&curr_node);
                        // Update is shorter
                        if new_score < nn.score {
                            nn.score = new_score;
                            nn.heuristic_score =
                                new_score + calculate_heuristic_score(nn.pos, d.goal);
                            nn.previous_pos = Some(curr_node.pos);
                        }
                    }
                }
            }
            if curr_node.pos == d.goal {
                let mut v: Vec<TilePos> = Vec::new();
                v.push(curr_node.pos);

                let mut otp = curr_node.previous_pos;
                while let Some(tp) = otp {
                    if tp != d.start {
                        v.push(tp);
                    }

                    otp = graph.get(&tp).expect("previous_pos node").previous_pos;
                }

                commands
                    .entity(e)
                    .remove::<Destination>()
                    .insert(TilePath(v));

                return;
            }
        }
    }
}

fn calculate_heuristic_score(curr: TilePos, target: TilePos) -> u32 {
    curr.0.abs_diff(target.0) + curr.1.abs_diff(target.1)
}

fn calculate_score(curr: &Node) -> u32 {
    curr.score + 1
}
