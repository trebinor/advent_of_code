use aoc_2019::day01::original_01a;
use aoc_2019::day01::original_01b;
use aoc_2019::day02::original_02a;
use aoc_2019::day02::original_02a_without_program_alarm_replacment;
use aoc_2019::day02::original_02b;
use aoc_2019::day03::original_03a;
use aoc_2019::day03::original_03b;
use aoc_2019::day04::original_04a;
use aoc_2019::day04::original_04b;
use aoc_2019::day04::parsed_04a;
use aoc_2019::day04::parsed_04b;
use aoc_2019::day05::icc_05a;
use aoc_2019::day05::icc_05b;
use aoc_2019::day05::original_05a;
use aoc_2019::day05::original_05b;
use aoc_2019::day06::original_06a;
use aoc_2019::day06::original_06b;
use aoc_2019::day07::original_07a;
use aoc_2019::day07::original_07b;
use aoc_2019::day08::original_08a;
use aoc_2019::day08::original_08b;
use aoc_2019::day09::original_09a;
use aoc_2019::day09::original_09b;
use aoc_2019::day10::generator as day10_generator;
use aoc_2019::day10::math_on_200th_asteroid;
use aoc_2019::day10::visible_asteroids;
use aoc_2019::day11::generator as day11_generator;
use aoc_2019::day11::paint_panels_start_black;
use aoc_2019::day11::paint_panels_start_white;
use aoc_2019::day12::generator as day12_generator;
use aoc_2019::day12::steps_to_repeat;
use aoc_2019::day12::total_energy_all_planets;
use aoc_2019::day13::original_13a;
use aoc_2019::day13::original_13b;
use aoc_2019::day14::generate_recipes;
use aoc_2019::day14::solution_14a;
use aoc_2019::day14::solution_14b;
use aoc_2019::day15::original_15a;
use aoc_2019::day15::original_15b;
use aoc_2019::day16::solution_16a;
use aoc_2019::day16::solution_16b;
use aoc_2019::day17::solution_17a;
use aoc_2019::day17::solution_17b;
use aoc_2019::day18::generator as day18_generator;
use aoc_2019::day18::shortest_path;
use aoc_2019::day18::shortest_path_with_quadbots;
use aoc_2019::day18::shortest_path_with_quadbots_ignore_doors;
use aoc_2019::day19::solution_19a;
use aoc_2019::day19::solution_19b;
use aoc_2019::day20::generator as day20_generator;
use aoc_2019::day20::shortest_layered_portal_path;
use aoc_2019::day20::shortest_portal_path;
use aoc_2019::day21::solution_21a;
use aoc_2019::day21::solution_21b;
use aoc_2019::day22::solution_22a;
use aoc_2019::day22::solution_22b;
use aoc_2019::day23::solution_23a;
use aoc_2019::day23::solution_23b;
use aoc_2019::day24::biodiversity_of_first_repeated_state;
use aoc_2019::day24::generator as day24_generator;
use aoc_2019::day24::total_bugs_200_iterations;
use aoc_2019::day25::solution_25a;
use std::fs;

macro_rules! test_gen {
    ($testname:ident, $funcname:ident, $gen:ident, $filepath:expr, $expected_result:expr) => {
        #[test]
        fn $testname() {
            assert_eq!(
                $expected_result,
                $funcname(&$gen(&fs::read_to_string($filepath).unwrap()))
            );
        }
    };
}

macro_rules! test_eq {
    ($testname:ident, $funcname:ident, $filepath:expr, $expected_result:expr) => {
        #[test]
        fn $testname() {
            assert_eq!(
                $expected_result,
                $funcname(&fs::read_to_string($filepath).unwrap().trim())
            );
        }
    };
}

macro_rules! test_ends_with {
    ($testname:ident, $funcname:ident, $filepath:expr, $expected_result:expr) => {
        #[test]
        fn $testname() {
            assert!($funcname(&fs::read_to_string($filepath).unwrap().trim())
                .ends_with($expected_result));
        }
    };
}

test_eq!(t01a_input, original_01a, "input/2019/01/input", 3_369_286);
test_eq!(t01a_example0, original_01a, "input/2019/01/example0", 2);
test_eq!(t01a_example1, original_01a, "input/2019/01/example1", 2);
test_eq!(t01a_example2, original_01a, "input/2019/01/example2", 654);
test_eq!(t01a_example3, original_01a, "input/2019/01/example3", 33583);
test_eq!(t01b_input, original_01b, "input/2019/01/input", 5_051_054);
test_eq!(t01b_example0, original_01b, "input/2019/01/example0", 2);
test_eq!(t01b_example1, original_01b, "input/2019/01/example1", 2);
test_eq!(t01b_example2, original_01b, "input/2019/01/example2", 966);
test_eq!(t01b_example3, original_01b, "input/2019/01/example3", 50346);

test_eq!(t02a_input, original_02a, "input/2019/02/input", 5_866_714);
test_eq!(
    t02a_example0,
    original_02a_without_program_alarm_replacment,
    "input/2019/02/example0",
    3500
);
test_eq!(t02b_input, original_02b, "input/2019/02/input", 5208);

test_eq!(t03a_input, original_03a, "input/2019/03/input", 557);
test_eq!(t03a_example0, original_03a, "input/2019/03/example0", 6);
test_eq!(t03a_example1, original_03a, "input/2019/03/example1", 159);
test_eq!(t03a_example2, original_03a, "input/2019/03/example2", 135);
test_eq!(t03b_input, original_03b, "input/2019/03/input", 56410);
test_eq!(t03b_example0, original_03b, "input/2019/03/example0", 30);
test_eq!(t03b_example1, original_03b, "input/2019/03/example1", 610);
test_eq!(t03b_example2, original_03b, "input/2019/03/example2", 410);

const ANSWER_04A: u32 = 921;
const ANSWER_04B: u32 = 603;
test_eq!(t04a_input, original_04a, "input/2019/04/input", ANSWER_04A);
test_eq!(
    t04a_input_parsed,
    parsed_04a,
    "input/2019/04/input",
    ANSWER_04A
);
test_eq!(t04b_input, original_04b, "input/2019/04/input", ANSWER_04B);
test_eq!(
    t04b_input_parsed,
    parsed_04b,
    "input/2019/04/input",
    ANSWER_04B
);

const ANSWER_05A: &str = "16489636";
const ANSWER_05B: &str = "9386583";
test_ends_with!(t05a_input, original_05a, "input/2019/05/input", ANSWER_05A);
test_eq!(t05a_input_icc, icc_05a, "input/2019/05/input", ANSWER_05A);
test_ends_with!(t05b_input, original_05b, "input/2019/05/input", ANSWER_05B);
test_ends_with!(t05b_input_icc, icc_05b, "input/2019/05/input", ANSWER_05B);

test_eq!(t06a_input, original_06a, "input/2019/06/input", 147_223);
test_eq!(t06a_example0, original_06a, "input/2019/06/example0", 42);
test_eq!(t06b_input, original_06b, "input/2019/06/input", 340);
test_eq!(t06b_example1, original_06b, "input/2019/06/example1", 4);

test_eq!(t07a_input, original_07a, "input/2019/07/input", 437_860);
test_eq!(t07a_example0, original_07a, "input/2019/07/example0", 43_210);
test_eq!(t07a_example1, original_07a, "input/2019/07/example1", 54_321);
test_eq!(t07a_example2, original_07a, "input/2019/07/example2", 65_210);
test_eq!(t07b_input, original_07b, "input/2019/07/input", 49_810_599);
test_eq!(t07b_example3, original_07b, "input/2019/07/example3", 139_629_729);
test_eq!(t07b_example4, original_07b, "input/2019/07/example4", 18_216);

test_eq!(t08a_input, original_08a, "input/2019/08/input", 1064);
test_eq!(t08b_input, original_08b, "input/2019/08/input", "PFCAK");

test_eq!(t09a_input, original_09a, "input/2019/09/input", 2_932_210_790);
test_eq!(t09b_input, original_09b, "input/2019/09/input", 73_144);

test_gen!(t10a_input, visible_asteroids, day10_generator, "input/2019/10/input", 263);
test_gen!(t10a_example0, visible_asteroids, day10_generator, "input/2019/10/example0", 33);
test_gen!(t10a_example1, visible_asteroids, day10_generator, "input/2019/10/example1", 35);
test_gen!(t10a_example2, visible_asteroids, day10_generator, "input/2019/10/example2", 41);
test_gen!(t10a_example3, visible_asteroids, day10_generator, "input/2019/10/example3", 210);
test_gen!(t10b_input, math_on_200th_asteroid, day10_generator, "input/2019/10/input", 1110);

test_gen!(t11a_input, paint_panels_start_black, day11_generator, "input/2019/11/input", 2088);
test_gen!(t11b_input, paint_panels_start_white, day11_generator, "input/2019/11/input", 249);

test_gen!(t12a_input, total_energy_all_planets, day12_generator, "input/2019/12/input", 9139);
test_gen!(t12b_input, steps_to_repeat, day12_generator, "input/2019/12/input", 420_788_524_631_496);
test_gen!(t12b_example0, steps_to_repeat, day12_generator, "input/2019/12/example0", 2772);
test_gen!(t12b_example1, steps_to_repeat, day12_generator, "input/2019/12/example1", 4_686_774_924);

test_eq!(t13a_input, original_13a, "input/2019/13/input", 427);
test_eq!(t13b_input, original_13b, "input/2019/13/input", 21_426);

test_gen!(t14a_input, solution_14a, generate_recipes, "input/2019/14/input", 443_537);
test_gen!(t14a_example0, solution_14a, generate_recipes, "input/2019/14/example0", 31);
test_gen!(t14a_example1, solution_14a, generate_recipes, "input/2019/14/example1", 165);
test_gen!(t14a_example2, solution_14a, generate_recipes, "input/2019/14/example2", 13_312);
test_gen!(t14a_example3, solution_14a, generate_recipes, "input/2019/14/example3", 180_697);
test_gen!(t14a_example4, solution_14a, generate_recipes, "input/2019/14/example4", 2_210_736);
test_gen!(t14b_input, solution_14b, generate_recipes, "input/2019/14/input", 2_910_558);
test_gen!(t14b_example2, solution_14b, generate_recipes, "input/2019/14/example2", 82_892_753);
test_gen!(t14b_example3, solution_14b, generate_recipes, "input/2019/14/example3", 5_586_022);
test_gen!(t14b_example4, solution_14b, generate_recipes, "input/2019/14/example4", 460_664);

test_eq!(t15a_input, original_15a, "input/2019/15/input", 204);
test_eq!(t15b_input, original_15b, "input/2019/15/input", 340);

test_eq!(t16a_input, solution_16a, "input/2019/16/input", "27831665");
test_eq!(t16a_example0, solution_16a, "input/2019/16/example0", "24176176");
test_eq!(t16a_example1, solution_16a, "input/2019/16/example1", "73745418");
test_eq!(t16a_example2, solution_16a, "input/2019/16/example2", "52432133");
test_eq!(t16b_input, solution_16b, "input/2019/16/input", "36265589");
test_eq!(t16b_example3, solution_16b, "input/2019/16/example3", "84462026");
test_eq!(t16b_example4, solution_16b, "input/2019/16/example4", "78725270");
test_eq!(t16b_example5, solution_16b, "input/2019/16/example5", "53553731");

test_eq!(t17a_input, solution_17a, "input/2019/17/input", 9544);
test_eq!(t17b_input, solution_17b, "input/2019/17/input", 1_499_679);

test_gen!(t18a_input, shortest_path, day18_generator, "input/2019/18/input", 4118);
test_gen!(t18a_example0, shortest_path, day18_generator, "input/2019/18/example0", 8);
test_gen!(t18a_example1, shortest_path, day18_generator, "input/2019/18/example1", 86);
test_gen!(t18a_example2, shortest_path, day18_generator, "input/2019/18/example2", 132);
test_gen!(t18a_example3, shortest_path, day18_generator, "input/2019/18/example3", 136);
test_gen!(t18a_example4, shortest_path, day18_generator, "input/2019/18/example4", 81);
test_gen!(t18b_input, shortest_path_with_quadbots_ignore_doors, day18_generator, "input/2019/18/input", 1828);
test_gen!(t18b_example5, shortest_path_with_quadbots, day18_generator, "input/2019/18/example5", 8);
test_gen!(t18b_example6, shortest_path_with_quadbots, day18_generator, "input/2019/18/example6", 24);
test_gen!(t18b_example7, shortest_path_with_quadbots, day18_generator, "input/2019/18/example7", 32);
test_gen!(t18b_example8, shortest_path_with_quadbots, day18_generator, "input/2019/18/example8", 72);

test_eq!(t19a_input, solution_19a, "input/2019/19/input", 226);
test_eq!(t19b_input, solution_19b, "input/2019/19/input", 7_900_946);

test_gen!(t20a_input, shortest_portal_path, day20_generator, "input/2019/20/input", 464);
test_gen!(t20a_example0, shortest_portal_path, day20_generator, "input/2019/20/example0", 23);
test_gen!(t20a_example1, shortest_portal_path, day20_generator, "input/2019/20/example1", 58);
test_gen!(t20b_input, shortest_layered_portal_path, day20_generator, "input/2019/20/input", 5802);
test_gen!(t20b_example2, shortest_layered_portal_path, day20_generator, "input/2019/20/example2", 396);

test_eq!(t21a_input, solution_21a, "input/2019/21/input", 19_349_530);
test_eq!(t21b_input, solution_21b, "input/2019/21/input", 1_142_805_439);

test_eq!(t22a_input, solution_22a, "input/2019/22/input", 2939);
test_eq!(t22b_input, solution_22b, "input/2019/22/input", 45_347_150_615_590);

test_eq!(t23a_input, solution_23a, "input/2019/23/input", 22_659);
test_eq!(t23b_input, solution_23b, "input/2019/23/input", 17_429);

test_gen!(t24a_input, biodiversity_of_first_repeated_state, day24_generator, "input/2019/24/input", 1_151_290);
test_gen!(t24a_example0, biodiversity_of_first_repeated_state, day24_generator, "input/2019/24/example0", 2_129_920);
test_gen!(t24b_input, total_bugs_200_iterations, day24_generator, "input/2019/24/input", 1953);

test_eq!(t25a_input, solution_25a, "input/2019/25/input", 1_090_617_344);
