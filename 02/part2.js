// gist: https://gist.github.com/uherman/0ff13dac129b2c65987a1da6546c554a
import fs from 'fs';

console.log(
  'Answer for part 2:',
  fs
    .readFileSync('input.txt', 'utf8')
    .split('\n')
    .reduce(
      (sum, line) =>
        ((game) =>
          (sum += Object.values(
            game.sets.reduce(
              (colors, set) => (
                set.map(
                  (cube) =>
                    (colors[cube.color] = Math.max(
                      colors[cube.color] || -Infinity,
                      parseInt(cube.number)
                    ))
                ),
                colors
              ),
              { red: -Infinity, green: -Infinity, blue: -Infinity }
            )
          ).reduce((acc, curr) => acc * curr, 1)))({
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
