const fs = require('fs');

const file_names = [ 'items', 'fluids', 'entities', 'recipes' ];

for (var file_name of file_names) {
    const file_path = '/home/mick/.factorio/script-output/' + file_name + '.json';
    const data = JSON.parse(fs.readFileSync(file_path, 'utf8'));
    fs.writeFileSync(file_name + '.json', JSON.stringify(data, null, 4));
}
