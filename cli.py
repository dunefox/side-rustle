#!/usr/bin/env python
import cli.app

@cli.app.CommandLineApp
def ls(app):
    pass

ls.add_param("-a", "--aaa", help="list in long format", default="asd")
ls.add_param("-b", "--bbb", help="list in long format", default=1)

if __name__ == "__main__":
    ls.run()

