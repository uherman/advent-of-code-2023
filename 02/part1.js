// gist: https://gist.github.com/uherman/2bc9ad39a126be404a4e77ee18eaa7be
import fs from 'fs';

const GAME_REQUIREMENTS = {
  red: 12,
  green: 13,
  blue: 14,
};

console.log(
  'Answer for part 1:',
  fs
    .readFileSync('input.txt', 'utf8')
    .split('\n')
    .reduce(
      (sum, line) =>
        ((game) =>
          game.sets.every((set) =>
            set.every((cube) => cube.number <= GAME_REQUIREMENTS[cube.color])
          )
            ? sum + game.id
            : sum)({
          id: parseInt(line.split(':')[0].match(/\d/g).join('')),
          sets: line
            .split(':')[1]
            .trim()
            .split(';')
            .map((set) =>
              set
                .split(',')
                .map((cube) =>
                  (([number, color]) => ({ number, color }))(
                    cube.trim().split(' ')
                  )
                )
            ),
        }),
      0
    )
);
