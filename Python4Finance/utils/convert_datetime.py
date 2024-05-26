import datetime

def convert_to_timestamp(date_time_str):
    # List of expected date and time formats, from most specific to least specific
    formats = [
        '%Y-%m-%d %H:%M:%S',  # Complete date and time
        '%Y-%m-%d %H:%M',     # Date and time without seconds
        '%Y-%m-%d %H',        # Date and hour only
        '%Y-%m-%d'            # Date only
    ]
    
    # Try parsing the date_time string using the expected formats
    for fmt in formats:
        try:
            date_time_obj = datetime.datetime.strptime(date_time_str, fmt)
            # Convert datetime object to timestamp (in seconds)
            timestamp = int(date_time_obj.timestamp())
            # Convert to milliseconds and ensure it is a 13-digit number
            timestamp_milliseconds = timestamp * 1000
            return '{:013d}'.format(timestamp_milliseconds)
        except ValueError:
            continue
    
    # Raise an error if no format matches
    raise ValueError("Date and time format is incorrect or unsupported.")