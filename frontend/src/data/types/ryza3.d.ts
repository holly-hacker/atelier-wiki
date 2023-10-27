// AUTO-GENERATED by typescript-type-def

export default types;
export namespace types {
    export type I32 = number;
    export type U32 = number;
    export type F32 = number;
    export type Item = {

        /**
         * The item tag. This is the closest we get to a string id but it does not exist for all items.
         */
        "tag": (string | null);
        "library_note": (string | null);
        "name": (string | null);
        "temp_name": (string | null);
        "temp_end_event": (string | null);
        "sort": types.I32;
        "img_no": types.I32;
        "price": types.U32;
        "lv": types.U32;
        "element": (types.U32 | null);
        "element_value": (types.U32 | null);
        "elem_fire": boolean;
        "elem_ice": boolean;
        "elem_thunder": boolean;
        "elem_air": boolean;
        "pc": (types.I32)[];
        "hp": (types.I32 | null);
        "atk": (types.I32 | null);
        "def": (types.I32 | null);
        "spd": (types.I32 | null);
        "w_hp": (types.F32 | null);
        "w_mp": (types.F32 | null);
        "w_atk": (types.F32 | null);
        "w_def": (types.F32 | null);
        "w_spd": (types.F32 | null);

        /**
         * The DLC required for this item.
         */
        "dlc": (string | null);
        "use_tag": string;
        "kind_tag": string;
        "bme": (string | null);
        "bmee": (string | null);
        "cat": (string)[];
    };
    export type ItemCategoryData = {
        "categories": Record<string, string>;
    };
    export type EffectAttribute = {
        "action": string;
        "attribute": (string | null);
        "min_1": (string | null);
        "max_1": (string | null);
        "min_2": (string | null);
        "max_2": (string | null);
    };
    export type ItemEffect = {

        /**
         * The name of the effect as it is shown in-game
         */
        "name": string;
        "description": (string | null);
        "kind": (string | null);
        "base_attribute": (string | null);
        "attributes": (types.EffectAttribute)[];
    };
    export type ItemEffectData = {
        "item_effects": Record<string, types.ItemEffect>;
    };
    export type RecipeIngredient = {

        /**
         * The item or category tag of this ingredient.
         */
        "tag": string;

        /**
         * Whether the tag refers to a category or an item.
         */
        "is_category": boolean;

        /**
         * The effect that is added by default, even if no material loops are filled in.
         */
        "initial_effect": (string | null);

        /**
         * The effect tags that this item adds.
         */
        "additional_effects": (string)[];
    };

    /**
     * The secret key info for a recipe.
     */
    export type SecretKeyInfo = {

        /**
         * The requirement for unlocking this level.
         * 
         * The type of requirement can be determined by the prefix of the string:
         * - `ITEM_ELEM_`: Element of the key
         * - `SECRET_KEY_MOTIF_`: Motif of the key
         * - `SECRET_KEY_RARITY_`: Rarity of the key
         */
        "requirement": string;

        /**
         * The reward for unlocking this level
         */
        "reward": string;
    };
    export type RingPredecessor = {

        /**
         * The direction of this node's predecessor.
         * 
         * This is an index in a clockwise direction, like so:
         * - 0: above (y - 2)
         * - 1: top right (x + 1, y - 1)
         * - 2: bottom right (x + 1, y + 1)
         * - 3: below (y + 2)
         * - 4: bottom left (x - 1, y + 1)
         * - 5: top left (x - 1, y - 1)
         */
        "direction": types.U32;

        /**
         * The requirement element value in the predecessor before this ring can be unlocked.
         */
        "required_value": (types.U32 | null);

        /**
         * The requirement element in the predecessor before this ring can be unlocked.
         */
        "required_element": (types.U32 | null);

        /**
         * The requirement quality in the recipe before this ring can be unlocked.
         */
        "required_quality": (types.U32 | null);
    };
    export type RingParameter = {

        /**
         * The value of the parameter. This could be a numeric value or a string that refers to an
         * item.
         */
        "value": string;

        /**
         * The element value required for this tier to be met. This is additive with the previous
         * tiers.
         */
        "element_value": types.U32;

        /**
         * Whether this element is hidden.
         */
        "hidden": boolean;
    };

    /**
     * A ring in a recipe, also called a material loop.
     */
    export type Ring = {

        /**
         * The element.
         * 
         * 0: Fire
         * 1: Ice
         * 2: Bolt
         * 3: Wind
         */
        "element": types.U32;

        /**
         * The effect type of the ring.
         * 
         * See `str_mix_feature_description` for more info.
         */
        "effect_type": types.U32;

        /**
         * Whether this is a core loop and must be filled in for the recipe to be completed.
         */
        "required": boolean;

        /**
         * The X coordinate, where negative is left and positive is right.
         */
        "x": types.I32;

        /**
         * The Y coordinate, where negative is up and positive is down.
         */
        "y": types.I32;

        /**
         * The pre-defined item or category to use, as defined in `itemrecipedata.xml`. Mutually
         * exclusive with `explicit_material`.
         */
        "restrict": (types.U32 | null);

        /**
         * The explicit material to use. This will be a different material than the 4 that are
         * pre-defined in the recipe. Mutually exclusive with `restrict`.
         */
        "explicit_material": (string | null);

        /**
         * The predecessor ring that this ring is connected to.
         * 
         * Each ring has at most 1 predecessor, and this predecessor must be unlocked before this ring
         * can be unlocked. There may also be additional requirements before it can be unlocked.
         */
        "predecessor": (types.RingPredecessor | null);

        /**
         * The parameters for the effects of this ring. Related to the `type`.
         */
        "effect_parameters": (types.RingParameter)[];
    };
    export type Recipe = {

        /**
         * The item that this recipe crafts.
         */
        "item_tag": string;

        /**
         * The sorting order of this recipe in the alchemy menu, if it is present there.
         */
        "sort": (types.I32 | null);

        /**
         * The base amount of the item that is crafted.
         */
        "make_num": types.U32;

        /**
         * The amount of time it takes to craft the item.
         */
        "hour": types.U32;

        /**
         * The category that this recipe is in
         */
        "recipe_category": string;

        /**
         * The core ingredients for this recipe.
         * 
         * At most 4 ingredients will be present, while the lower limit is likely 3.
         */
        "ingredients": (types.RecipeIngredient)[];

        /**
         * The secret key info for this recipe, if it is applicable.
         */
        "secret_key_info": ([types.SecretKeyInfo, types.SecretKeyInfo, types.SecretKeyInfo] | null);

        /**
         * The fields of this recipe, each containing a set of rings/material loops..
         */
        "fields": ((types.Ring)[])[];
    };
    export type FeatureDescription = {

        /**
         * The name of the effect.
         */
        "short_name": (string | null);
        "short_description": (string | null);

        /**
         * The format string for the effect, as shown in the Loop Info window.
         * 
         * For effects 1-4, this will be `null` and the item effect's descripion will be used instead.
         */
        "loop_info_format": (string | null);
        "description": (string | null);
    };
    export type RecipeData = {
        "recipes": (types.Recipe)[];

        /**
         * A lookup table for feature descriptions.
         */
        "feature_descriptions": Record<types.U32, types.FeatureDescription>;
    };
    export type Usize = number;
    export type FieldMap = {
        "field_map_name": (string | null);
        "data_file_name": string;
        "load_region": (string | null);
        "range_min_x": types.Usize;
        "range_min_z": types.Usize;
        "range_max_x": types.Usize;
        "range_max_z": types.Usize;
        "navi_range_min_x": (types.Usize | null);
        "navi_range_min_z": (types.Usize | null);
        "navi_range_max_x": (types.Usize | null);
        "navi_range_max_z": (types.Usize | null);
        "area_tag": string;
        "region_tag": string;
        "qua_min": (types.Usize | null);
        "qua_max": (types.Usize | null);
        "num_min": (types.Usize | null);
        "num_max": (types.Usize | null);
        "grade_min": (types.Usize | null);
        "grade_max": (types.Usize | null);
    };
    export type RegionMap = {
        "image_name": string;
        "rot": [types.F32, types.F32, types.F32];
        "pos": [types.F32, types.F32, types.F32];
        "scale": [types.F32, types.F32, types.F32];
    };
    export type FieldMapData = {
        "field_maps": (types.FieldMap)[];
        "region_maps": Record<types.Usize, types.RegionMap>;
    };
    export type GimmickData = {

        /**
         * The position of this gimmick.
         */
        "position": [types.F32, types.F32, types.F32];

        /**
         * The chance of this gimmick being placed, between 0 and 100 inclusive.
         */
        "rate": types.Usize;

        /**
         * Whether the state of this gimmick is saved.
         */
        "save": boolean;
    };
    export type CutDownTree = (types.GimmickData & {

        /**
         * Drop information when using rod.
         */
        "rod": [(string | null), (types.Usize | null)];

        /**
         * Drop information when using sickle.
         */
        "sickle": [(string | null), (types.Usize | null)];

        /**
         * Drop information when using axe.
         */
        "axe": [(string | null), (types.Usize | null)];

        /**
         * Drop information when using hammer.
         */
        "hammer": [(string | null), (types.Usize | null)];
    });
    export type EnemyRandomSpawner = (types.GimmickData & {
        "min": types.Usize;
        "max": types.Usize;
        "symbol_group_1": (string | null);
        "symbol_group_2": (string | null);
        "symbol_group_3": (string | null);
        "symbol_group_4": (string | null);
        "symbol_group_5": (string | null);
        "monster_count": (types.Usize | null);
        "monster": (string | null);
    });
    export type InstantEnemySpawner = (types.GimmickData & {
        "symbol_group": string;
        "min_count": (types.Usize | null);
    });
    export type FieldDataSet = {

        /**
         * Trees that can be cut down for resources.
         */
        "cut_down_tree": (types.CutDownTree)[];

        /**
         * Random spawn points for enemies.
         */
        "enemy_random_spawner": (types.EnemyRandomSpawner)[];

        /**
         * Instant spawn points for enemies. Presumably used for boss monsters.
         */
        "instant_enemy_spawner": (types.InstantEnemySpawner)[];
    };
    export type FieldData = Record<string, types.FieldDataSet>;
    export type EnemyDrop = {
        "item_tag": string;
        "rate": types.U32;
        "num": types.U32;
        "quality_min": types.F32;
        "quality_max": types.F32;
        "potential_min": types.F32;
        "potential_max": types.F32;
        "potential_num_min": types.F32;
        "potential_num_max": types.F32;
        "potential_lv_min": types.F32;
        "potential_lv_max": types.F32;
        "quality_min_adj": types.F32;
        "quality_max_adj": types.F32;
        "potential_min_adj": types.F32;
        "potential_max_adj": types.F32;
        "potential_num_min_adj": types.U32;
        "potential_num_max_adj": types.U32;
        "potential_lv_min_adj": types.U32;
        "potential_lv_max_adj": types.U32;
        "super_pot_rate": types.U32;
        "factor": string;
        "eff": (string | null);
    };
    export type EnemyStatus = {
        "exp": types.U32;
        "money": types.U32;
        "exp_rosca": types.U32;
        "money_rosca": types.U32;
        "gold_coin": types.U32;
        "gold_coin_rate": types.U32;
        "drop_tag": string;
        "skill_tag": string;
        "extra_skill_tag": string;
        "lv": types.U32;
        "stun": types.U32;
        "key_make": types.U32;
        "atk_num": types.U32;
        "burst_up": types.U32;
        "burst_max": types.U32;
        "hp": types.I32;
        "atk": types.I32;
        "def": types.I32;
        "spd": types.I32;
        "bad_resist": (types.U32)[];
        "resist_non": types.U32;
        "key_create_tag": string;
        "att": (string)[];

        /**
         * `sp_item_tag` from drop data
         */
        "sp_item_tag": string;
        "drops": (types.EnemyDrop)[];
    };
    export type Enemy = {
        "name": string;
        "library_note": (string | null);
        "is_big": boolean;
        "img_no": types.I32;
        "wait_action": boolean;
        "library_rank_health": types.U32;
        "library_rank_attack": types.U32;
        "library_rank_speed": types.U32;
        "library_rank_defense": types.U32;
        "dlc": (string | null);
        "shoot_up": boolean;
        "monster_tag": string;
        "chara_tag": string;
        "race_tag": string;
        "size": string;
        "division": string;
        "statusses": (types.EnemyStatus)[];
    };
    export type PuniFeedingEventCondition = ({

        /**
         * A specific puni species.
         */
        "PuniSpecies": string;
    } | {

        /**
         * A range for the energy value.
         */
        "Energy": [types.U32, types.U32];
    } | {

        /**
         * A range for the color value.
         */
        "Color": [types.U32, types.U32];
    } | {

        /**
         * A range for the mood value.
         */
        "Mood": [types.U32, types.U32];
    });

    /**
     * A feeding event where a unique item is awarded.
     */
    export type PuniFeedingUniqueEvent = {

        /**
         * The item tag
         */
        "item_tag": string;

        /**
         * The condition required to trigger this event.
         */
        "condition": types.PuniFeedingEventCondition;
    };

    /**
     * A puni species that can result from feeding the puni.
     */
    export type PuniFeedingSpecies = {

        /**
         * The name of this species
         */
        "name": string;
        "character_tag": string;
        "image_no": types.Usize;

        /**
         * The energy range for this species.
         */
        "energy": [types.U32, types.U32];

        /**
         * The color range for this species.
         */
        "color": [types.U32, types.U32];

        /**
         * The mood range for this species.
         */
        "mood": [types.U32, types.U32];
        "rank_e": ([types.U32, types.U32] | null);
        "rank_d": ([types.U32, types.U32] | null);
        "rank_c": ([types.U32, types.U32] | null);
        "rank_b": ([types.U32, types.U32] | null);
        "rank_a": ([types.U32, types.U32] | null);
        "rank_s": ([types.U32, types.U32] | null);
        "categories": (string)[];
    };
    export type PuniFeedingData = {
        "unique_events": (types.PuniFeedingUniqueEvent)[];
        "species": (types.PuniFeedingSpecies)[];
    };
    export type Ryza3Data = {
        "item_data": (types.Item)[];
        "item_category_data": types.ItemCategoryData;
        "item_effect_data": types.ItemEffectData;
        "recipe_data": types.RecipeData;
        "field_map": types.FieldMapData;
        "field_data": types.FieldData;
        "enemy_data": (types.Enemy)[];
        "puni_feeding_data": types.PuniFeedingData;
    };
}
