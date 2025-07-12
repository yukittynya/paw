git pull origin main 
git add .

echo "Message qt3.14?"
read msg 

git commit -m "${msg}"
git push origin main
