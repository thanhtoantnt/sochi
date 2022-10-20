python3 script/extract.py --proj /home/thanhtoantnt/workspace/sochi/ilf-data/buggy_1 --port 8545
python3 -m ilf --proj /home/thanhtoantnt/workspace/sochi/ilf-data/buggy_1 --contract EIP20Interface --fuzzer imitation --model ./model/ --limit 2000
python3 -m ilf --proj /home/thanhtoantnt/workspace/sochi/ilf-data/buggy_1 --contract HotDollarsToken --fuzzer imitation --model ./model/ --limit 2000

