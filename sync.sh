fswatch -0 ./debugger | while read -d "" event 
do 
  rsync -avz ./debugger gochess:~/debugger
done
