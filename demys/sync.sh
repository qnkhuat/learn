fswatch -0 ./ | while read -d "" event 
do 
  rsync -avz ./ gochess:~/temp/demys
done
