const {glob} = require('glob');
const {exec} = require('child_process');

// 👇 Find every scoped SCSS file
glob("**/*.razor.scss", function (er, files) {
    files.forEach(function (file) {
        // 👇 Output the processed file as *.razor.css, which is a normal scoped CSS file
        const command = `npx postcss "${file}" -o "${file.replace('.razor.scss', '.razor.css')}"`;
        console.log(command)
        exec(command, (error, stdout, stderr) => {
            if (error) {
                console.error(`exec error: ${error}`);
                return;
            }
            console.log(`stdout: ${stdout}`);
            console.error(`stderr: ${stderr}`);
        });
    });
})
