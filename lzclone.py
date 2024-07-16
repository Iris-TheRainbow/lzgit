from sys import argv
from os import system

args = argv
args.pop(0)
if args[0] == '--help':
    print('lzclone: cloning for lazy people. It clones then automatically changes to that directory')
def getname(url):
    seperated = url.split('/')
    name = seperated[-1].split('.')[0]
    return name

system('git clone ' + args[0])
system('cd ' + getname(args[0]))
