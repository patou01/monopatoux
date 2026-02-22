from time import sleep

from context_timing import measure_time, set_log_func

set_log_func(print)
with measure_time() as m:
    sleep(0.1)
    m.print()
    sleep(0.1)
