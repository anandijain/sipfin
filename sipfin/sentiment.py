from transformers import pipeline
import pandas as pd

def add_sentiment_df(s: pd.Series, label_col="sentiment_label", score_col="sentiment_score") -> pd.DataFrame:
    nlp = pipeline('sentiment-analysis')
    sentiments = [nlp(text)[0] for text in s]

    labels = [s['label'] for s in sentiments]
    labels = pd.Series(labels, name=label_col)

    scores = [s['score'] for s in sentiments]
    scores = pd.Series(scores, name=score_col)
    return pd.concat([s, labels, scores], axis=1)
