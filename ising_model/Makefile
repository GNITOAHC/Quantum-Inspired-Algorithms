CC = g++
CFLAGS = -Wall -g -std=c++20

TARGET = main

all: $(TARGET)

# $(TARGET): $(TARGET).c
# 	$(CC) $(CFLAGS) -o $(TARGET) $(TARGET).c

$(TARGET): main.o Jxx.h Nodes.o Node.o IsingModel.o
	$(CC) $(CFLAGS) -o main main.o Nodes.o Node.o IsingModel.o

main.o: main.cpp Nodes.h IsingModel.h
	$(CC) $(CFLAGS) -c main.cpp

Nodes.o: Nodes.cpp Nodes.h Node.h Jxx.h Direction.h
	$(CC) $(CFLAGS) -c Nodes.cpp

Node.o: Node.cpp Node.h Jxx.h SubLattice.h Spin.h
	$(CC) $(CFLAGS) -c Node.cpp

IsingModel.o: IsingModel.cpp IsingModel.h Nodes.h
	$(CC) $(CFLAGS) -c IsingModel.cpp

clean:
	$(RM) $(TARGET) *.o
