import exogress
import logging
from aiohttp import web

# LOGGER_NAME = 'exogress'
#
# logger = logging.getLogger(LOGGER_NAME)
#
# formatter = logging.Formatter('%(asctime)s : %(levelname)s : %(name)s : %(message)s')
#
# terminal = logging.StreamHandler()
# terminal.setFormatter(formatter)
#
# logger.addHandler(terminal)

# logger.info("serving on 3000")

exogress.spawn(
    access_key_id="01EH3CBPYHBCFJ5MWM1E5N888Z",
    secret_access_key="2eoAYVjpjtztomf7mL94fJeZVS5TSkvEDSYB97v1CQxDDyfeg4edVpPDgGxKiou2s2c3wu54WNKdvXo4TqwRffN8MVHxP6w2dPzy8MQz8nWtSf5yhzHjhUidVH8c3Q4ZCoUGYoXicZHGMTrRbw1D2tiie3Sc6GeKdyT9zN15CeVqhiJc9AtJbSCvmX9jJ",
    account="glebpom",
    project="home"
)


async def handle(request):
    return web.Response(text="Hello from exogress on python")


app = web.Application()
app.router.add_get('/', handle)

web.run_app(app, port=4000)
