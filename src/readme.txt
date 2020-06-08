Readme Instructions for the Game



OTHER NOTES
Probably more of a note for myself but here's the steps to commit any project:
1. Make a repository in GitHub.
2. In my local directory of code in the terminal, do the following:

git init
git add .
git commit -m 'first commit'
git remote add origin https://github.com/shenge86/guessing_game.git
git push -u origin master

2a. Note that sometimes the last line doesn't work. In that case, do this:
git pull origin master --allow-unrelated-histories

2b. Note that to check status at any time, you can do:
git remote
(which should return origin)

git status
(which should display what files are commited already or not yet committed)
