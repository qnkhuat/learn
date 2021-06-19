fswatch -0 ./ | while read -d "" event 
do 
  rsync -avz ./src gochess:~/temp/demys/
done
